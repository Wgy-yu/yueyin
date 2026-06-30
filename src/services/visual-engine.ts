import * as THREE from "three";
import type { AudioAnalysis } from "./audio-engine";

// Inline shaders — ponytail: single file, no .glsl loader dependency
const vertexShader = `
uniform float uTime;
uniform float uBass;
uniform float uMid;
uniform float uTreble;
uniform float uEnergy;
uniform vec2 uMouse;
uniform float uMouseActive;
attribute vec2 aUv;
attribute float aRand;
varying float vAlpha;
varying vec3 vColor;

void main() {
  vec3 pos = position;

  // Bass-driven breathing
  float breath = 1.0 + uBass * 0.35;
  pos.xy *= breath;

  // Treble jitter
  pos.x += sin(uTime * 3.7 + aRand * 28.0) * uTreble * 0.12;
  pos.y += cos(uTime * 4.1 + aRand * 32.0) * uTreble * 0.10;

  // Mid-driven Z displacement
  pos.z += sin(aUv.x * 6.28 + uTime * 1.2) * uMid * 0.25;
  pos.z += cos(aUv.y * 6.28 + uTime * 0.9) * uMid * 0.18;

  // Mouse repulsion (matching Mineradio's Silk preset approach)
  vec2 delta = pos.xy - uMouse;
  float dist = length(delta);
  float mouseInfluence = uMouseActive * smoothstep(1.5, 0.0, dist);
  pos.xy += normalize(delta + 0.001) * mouseInfluence * 0.6;
  pos.z += mouseInfluence * 0.4;

  // Color from UV + audio energy
  vColor = mix(
    vec3(0.0, 0.96, 0.83),   // cyan
    vec3(1.0, 0.33, 0.4),    // red
    aUv.x
  );
  vColor = mix(vColor, vec3(1.0, 0.84, 0.28), uEnergy * 0.3); // gold tint

  vAlpha = 0.5 + uEnergy * 0.5;

  vec4 mvPosition = modelViewMatrix * vec4(pos, 1.0);
  gl_Position = projectionMatrix * mvPosition;
  gl_PointSize = (2.5 + uBass * 3.0) * (300.0 / max(length(mvPosition.xyz), 0.1));
}
`;

const fragmentShader = `
varying float vAlpha;
varying vec3 vColor;

void main() {
  float d = length(gl_PointCoord - 0.5);
  float alpha = smoothstep(0.5, 0.2, d) * vAlpha;
  gl_FragColor = vec4(vColor, alpha);
}
`;

export class VisualEngine {
  private renderer: THREE.WebGLRenderer | null = null;
  private scene: THREE.Scene | null = null;
  private camera: THREE.PerspectiveCamera | null = null;
  private particles: THREE.Points | null = null;
  private material: THREE.ShaderMaterial | null = null;
  private uniforms: Record<string, THREE.IUniform> | null = null;
  private mouse = new THREE.Vector2(0, 0);
  private mouseActive = 0;
  private animId: number | null = null;
  private prevTime = 0;
  private paused = false;

  mount(container: HTMLElement) {
    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(45, innerWidth / innerHeight, 0.1, 100);
    this.camera.position.z = 4.5;

    this.renderer = new THREE.WebGLRenderer({
      antialias: false,
      alpha: true,
      powerPreference: "high-performance",
    });
    // ponytail: adaptive DPR from splash screen optimization, range 0.85–1.35
    const dpr = Math.min(1.35, Math.max(0.85, devicePixelRatio));
    this.renderer.setPixelRatio(dpr);
    this.renderer.setSize(innerWidth, innerHeight);
    container.appendChild(this.renderer.domElement);

    this.buildParticles();
    this.bindEvents(container);
    this.prevTime = performance.now();
    this.animate();
  }

  private buildParticles() {
    const COUNT = 4800;
    const PLANE = 4.8;
    const positions = new Float32Array(COUNT * 3);
    const uvs = new Float32Array(COUNT * 2);
    const rands = new Float32Array(COUNT);
    const side = Math.sqrt(COUNT);

    for (let i = 0; i < COUNT; i++) {
      const ix = i % side;
      const iy = Math.floor(i / side);
      const u = ix / side;
      const v = iy / side;
      positions[i * 3] = (u - 0.5) * PLANE;
      positions[i * 3 + 1] = (v - 0.5) * PLANE;
      positions[i * 3 + 2] = 0;
      uvs[i * 2] = u;
      uvs[i * 2 + 1] = v;
      rands[i] = Math.random();
    }

    const geo = new THREE.BufferGeometry();
    geo.setAttribute("position", new THREE.BufferAttribute(positions, 3));
    geo.setAttribute("aUv", new THREE.BufferAttribute(uvs, 2));
    geo.setAttribute("aRand", new THREE.BufferAttribute(rands, 1));

    this.uniforms = {
      uTime: { value: 0 },
      uBass: { value: 0 },
      uMid: { value: 0 },
      uTreble: { value: 0 },
      uEnergy: { value: 0 },
      uMouse: { value: new THREE.Vector2(0, 0) },
      uMouseActive: { value: 0 },
    };

    this.material = new THREE.ShaderMaterial({
      vertexShader,
      fragmentShader,
      uniforms: this.uniforms,
      transparent: true,
      depthWrite: false,
      blending: THREE.AdditiveBlending,
    });

    this.particles = new THREE.Points(geo, this.material);
    this.particles.frustumCulled = false;
    this.scene!.add(this.particles);
  }

  private bindEvents(container: HTMLElement) {
    const onResize = () => {
      if (!this.camera || !this.renderer) return;
      this.camera.aspect = innerWidth / innerHeight;
      this.camera.updateProjectionMatrix();
      this.renderer.setSize(innerWidth, innerHeight);
    };
    const onMove = (e: PointerEvent) => {
      const rect = container.getBoundingClientRect();
      this.mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
      this.mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;
      this.mouseActive = 1;
    };
    const onLeave = () => { this.mouseActive = 0; };
    const onVisibility = () => {
      if (document.hidden) {
        this.paused = true;
        if (this.animId) { cancelAnimationFrame(this.animId); this.animId = null; }
      } else {
        this.paused = false;
        this.prevTime = performance.now();
        this.animate();
      }
    };

    addEventListener("resize", onResize);
    container.addEventListener("pointermove", onMove);
    container.addEventListener("pointerleave", onLeave);
    document.addEventListener("visibilitychange", onVisibility);

    this._cleanup = () => {
      removeEventListener("resize", onResize);
      container.removeEventListener("pointermove", onMove);
      container.removeEventListener("pointerleave", onLeave);
      document.removeEventListener("visibilitychange", onVisibility);
    };
  }

  private _cleanup: (() => void) | null = null;

  update(analysis: AudioAnalysis) {
    if (!this.uniforms) return;
    this.uniforms.uBass.value = analysis.bass;
    this.uniforms.uMid.value = analysis.mid;
    this.uniforms.uTreble.value = analysis.treble;
    this.uniforms.uEnergy.value = analysis.energy;
    this.uniforms.uMouse.value.copy(this.mouse);
    this.uniforms.uMouseActive.value += (this.mouseActive - this.uniforms.uMouseActive.value) * 0.08;
  }

  private animate = () => {
    if (this.paused) return;
    this.animId = requestAnimationFrame(this.animate);
    const now = performance.now();
    const dt = Math.min(0.05, (now - this.prevTime) / 1000);
    this.prevTime = now;
    if (this.uniforms) this.uniforms.uTime.value += dt;

    // Gentle camera sway
    if (this.camera) {
      const t = this.uniforms?.uTime.value ?? 0;
      this.camera.position.x = Math.sin(t * 0.15) * 0.3;
      this.camera.position.y = Math.cos(t * 0.12) * 0.2;
      this.camera.lookAt(0, 0, 0);
    }

    this.renderer?.render(this.scene!, this.camera!);
  };

  dispose() {
    this.paused = true;
    if (this.animId) { cancelAnimationFrame(this.animId); this.animId = null; }
    this._cleanup?.();
    this.particles?.geometry.dispose();
    this.material?.dispose();
    this.renderer?.dispose();
    this.renderer?.domElement.remove();
    this.renderer = null;
    this.scene = null;
    this.camera = null;
    this.particles = null;
    this.material = null;
    this.uniforms = null;
  }
}
