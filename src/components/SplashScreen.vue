<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

const emit = defineEmits<{
  (e: "enter"): void;
}>();

const splashState = ref<"loading" | "ready" | "exiting">("loading");
const canvasRef = ref<HTMLCanvasElement | null>(null);
let animationId: number | null = null;
let startTime = 0;

interface Dust {
  x: number;
  y: number;
  vx: number;
  vy: number;
  r: number;
  a: number;
  p: number;
}

interface Streak {
  x: number;
  y: number;
  len: number;
  width: number;
  speed: number;
  angle: number;
  phase: number;
  color: string;
  delay: number;
  alpha: number;
}

interface Shard {
  ox: number;
  oy: number;
  w: number;
  h: number;
  skew: number;
  phase: number;
  color: string;
  alpha: number;
}

let dust: Dust[] = [];
let streaks: Streak[] = [];
let shards: Shard[] = [];
let canvasW = 0;
let canvasH = 0;
let resizeHandler: (() => void) | null = null;

function initWebGL(canvas: HTMLCanvasElement): (() => void) | null {
  const gl = canvas.getContext("webgl", { alpha: true, antialias: false, depth: false,
    stencil: false, premultipliedAlpha: false, powerPreference: "high-performance" }) as WebGLRenderingContext | null;
  if (!gl) return null;
  const vertex = `attribute vec2 aPosition;varying vec2 vUv;void main(){vUv=aPosition*.5+.5;gl_Position=vec4(aPosition,0.,1.);}`;
  const fragment = `precision highp float;
varying vec2 vUv;uniform vec2 uResolution;uniform float uTime;
float sat(float v){return clamp(v,0.,1.);}float ease(float v){v=sat(v);return v*v*(3.-2.*v);}
mat2 rot(float a){float c=cos(a),s=sin(a);return mat2(c,-s,s,c);}
float hash(vec2 p){return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453123);}
float noise(vec2 p){vec2 i=floor(p),f=fract(p),u=f*f*(3.-2.*f);return mix(mix(hash(i),hash(i+vec2(1,0)),u.x),mix(hash(i+vec2(0,1)),hash(i+vec2(1)),u.x),u.y);}
float loop(vec2 q,float t,float ch){q*=rot(.28+sin(t*.18)*.12);q+=vec2(.055*sin(t*.3+ch),.04*cos(t*.24+ch*1.7));float a=atan(q.y,q.x);float nd=length(q)+sin(a*3.+t*.72+ch*1.9)*.078+sin(a*7.-t*.54+ch)*.02;float wd=length(q*vec2(1.34+.06*sin(t*.25),.82+.04*cos(t*.31)))+.026*sin(q.x*4.4+t*.62)+.018*sin(q.y*5.2-t*.45);float d=mix(mix(wd,abs(q.x)*1.2+abs(q.y)*.84,.32),nd,.2+.04*sin(t*.18+ch));float pat=mod((q.x+q.y)*.62+sin(q.x*5.5+t)*.015+sin(q.y*7.-t*.75)*.012,.2),acc=0.;for(int i=1;i<=6;i++){float f=float(i);acc+=.0011*f*f/max(abs(fract(t*.152-ch*.018+.011*f)*4.7-d+pat),.0065);}float tc=q.x*.92-q.y*.58+.03*sin(q.x*5.2+t*.72);acc+=.0065/max(abs(sin((tc+t*.1+ch*.035)*27.)),.07)*(.5+.3*sin(a*1.2+t+ch));return min(acc,1.95);}
void main(){vec2 p=vUv*2.-1.;p.x*=uResolution.x/max(uResolution.y,1.);float t=uTime,intro=ease(t/.72),bloom=ease((t-.1)/1.1),climax=exp(-pow((t-3.62)/.58,2.)),pre=ease((t-2.15)/1.25)*(1.-ease((t-3.86)/.72)),after=exp(-pow((t-4.14)/.62,2.)),calm=1.-.22*ease((t-4.75)/.7),settle=1.-.34*ease((t-5.05)/.52);vec2 uv=p*(.98+.05*sin(t*.25))+vec2(0,-.025),fa=normalize(vec2(.86,-.5)),ca=vec2(-fa.y,fa.x);float lane=dot(p,fa),cross=dot(p,ca);uv+=fa*sin(cross*5.4+lane*1.1-t*1.85)*.055*climax+ca*sin(lane*7.2+t*1.25)*.034*climax;uv*=1.+.045*pre-.02*climax;vec3 r=vec3(1,.13,.31),g=vec3(.16,1,.86),y=vec3(1,.76,.28);vec3 lc=r*loop(uv,t,0.)+g*loop(uv*1.018+vec2(.012,-.008),t+.18,1.)+y*loop(uv*.986+vec2(-.01,.01),t+.35,2.);lc+=mix(g,y,.35+.25*sin(t))*loop(uv*1.42+vec2(sin(t*.2)*.08,cos(t*.17)*.05),t*1.12+1.7,2.7)*(.3+.24*pre);float band=exp(-pow((lane+.08*sin(t*.72))/.62,2.));vec3 cc=(mix(g,y,.36)*pow(.5+.5*sin(cross*13.5+lane*2.2-t*3.1),8.)+r*pow(.5+.5*sin(cross*9.-lane*5.4+t*2.4),10.)*.52)*band*climax;cc+=mix(r,g,vUv.x)*exp(-pow((lane-.34)/.72,2.))*after*.13;float beam=exp(-abs(p.y+.005*sin(t*3.))*24.)*(.14+.52*exp(-pow((t-.74)/.34,2.))),mask=smoothstep(-1.55,-.08,p.x)*(1.-smoothstep(.08,1.55,p.x));vec3 col=vec3(.002,.004,.005)+lc*(.56+.46*bloom)*calm*settle+cc*.22+mix(r,g,vUv.x)*beam*mask*(.4+.28*climax);col+=vec3(1,.78,.42)*exp(-dot(p,p)*3.6)*exp(-pow((t-.88)/.4,2.))*.18;col*=.92+.08*sin((vUv.y*uResolution.y+t*52.)*.72);col+=(noise(vUv*uResolution.xy*.52+t*17.)-.5)*.018;col*=intro;col=max(col-vec3(.01,.012,.012),0.);col=vec3(1)-exp(-col*(.62+.18*climax));float vig=smoothstep(1.52,.2,length(p*vec2(.78,1.04)));col*=.38+.86*vig;col+=vec3(.02,.01,.014)*(1.-vig);gl_FragColor=vec4(col,1);}`;
  const compile = (type: number, source: string) => { const s = gl.createShader(type)!; gl.shaderSource(s, source); gl.compileShader(s); if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) { console.warn(gl.getShaderInfoLog(s)); return null; } return s; };
  const vs = compile(gl.VERTEX_SHADER, vertex), fs = compile(gl.FRAGMENT_SHADER, fragment);
  if (!vs || !fs) return null;
  const program = gl.createProgram()!; gl.attachShader(program, vs); gl.attachShader(program, fs); gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) return null;
  const buffer = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, buffer); gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1,-1,3,-1,-1,3]), gl.STATIC_DRAW);
  const position = gl.getAttribLocation(program, "aPosition"), resolution = gl.getUniformLocation(program, "uResolution"), time = gl.getUniformLocation(program, "uTime");
  const resize = () => { const dpr=Math.min(1.6,Math.max(1,devicePixelRatio||1)); canvas.width=Math.floor(innerWidth*dpr);canvas.height=Math.floor(innerHeight*dpr);gl.viewport(0,0,canvas.width,canvas.height); };
  resizeHandler=resize; resize(); addEventListener("resize",resize); startTime=performance.now();
  const draw=()=>{ const elapsed=(performance.now()-startTime)/1000;gl.useProgram(program);gl.bindBuffer(gl.ARRAY_BUFFER,buffer);gl.enableVertexAttribArray(position);gl.vertexAttribPointer(position,2,gl.FLOAT,false,0,0);gl.uniform2f(resolution,canvas.width,canvas.height);gl.uniform1f(time,elapsed);gl.drawArrays(gl.TRIANGLES,0,3);animationId=requestAnimationFrame(draw);}; draw();
  return draw;
}

function smoothstep(edge0: number, edge1: number, x: number): number {
  const t = Math.max(0, Math.min(1, (x - edge0) / Math.max(0.0001, edge1 - edge0)));
  return t * t * (3 - 2 * t);
}

function easeOutCubic(t: number): number {
  t = Math.max(0, Math.min(1, t));
  return 1 - Math.pow(1 - t, 3);
}

function initCanvas(canvas: HTMLCanvasElement) {
  if (!window.matchMedia("(prefers-reduced-motion: reduce)").matches && initWebGL(canvas)) return;
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  const resize = () => {
    const dpr = Math.min(1.6, Math.max(1, window.devicePixelRatio || 1));
    canvasW = window.innerWidth;
    canvasH = window.innerHeight;
    canvas.width = Math.max(1, Math.floor(canvasW * dpr));
    canvas.height = Math.max(1, Math.floor(canvasH * dpr));
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    // Init dust particles
    dust = [];
    for (let i = 0; i < 84; i++) {
      dust.push({
        x: Math.random() * canvasW,
        y: Math.random() * canvasH,
        vx: (Math.random() - 0.5) * 0.18,
        vy: (Math.random() - 0.5) * 0.11,
        r: Math.random() * 1.35 + 0.28,
        a: Math.random() * 0.105 + 0.025,
        p: Math.random() * Math.PI * 2,
      });
    }

    // Init streaks
    const streakColors = [
      "rgba(244,210,138,",
      "rgba(122,215,194,",
      "rgba(255,83,103,",
      "rgba(157,184,207,",
    ];
    streaks = [];
    for (let s = 0; s < 22; s++) {
      streaks.push({
        x: Math.random() * canvasW,
        y: canvasH * (0.2 + Math.random() * 0.62),
        len: canvasW * (0.12 + Math.random() * 0.24),
        width: 0.75 + Math.random() * 2.1,
        speed: canvasW * (0.00028 + Math.random() * 0.00042),
        angle: ((-10 + Math.random() * 20) * Math.PI) / 180,
        phase: Math.random() * Math.PI * 2,
        color: streakColors[s % streakColors.length],
        delay: Math.random() * 1.1,
        alpha: 0.18 + Math.random() * 0.36,
      });
    }

    // Init shards
    shards = [];
    for (let h = 0; h < 34; h++) {
      shards.push({
        ox: (Math.random() - 0.5) * canvasW * 0.92,
        oy: (Math.random() - 0.5) * canvasH * 0.22,
        w: 18 + Math.random() * 86,
        h: 1 + Math.random() * 5,
        skew: (Math.random() - 0.5) * 20,
        phase: Math.random() * Math.PI * 2,
        color: streakColors[h % streakColors.length],
        alpha: 0.1 + Math.random() * 0.24,
      });
    }
  };

  resize();
  resizeHandler = resize;
  window.addEventListener("resize", resize);
  startTime = performance.now();

  animate(canvas, ctx);
}

function animate(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D) {
  const elapsed = (performance.now() - startTime) / 1000;

  ctx.clearRect(0, 0, canvasW, canvasH);

  // Background gradient
  const base = ctx.createLinearGradient(0, 0, canvasW, canvasH);
  base.addColorStop(0, "rgba(1,6,7,0.68)");
  base.addColorStop(0.45, "rgba(10,9,12,0.74)");
  base.addColorStop(1, "rgba(0,0,0,0.84)");
  ctx.fillStyle = base;
  ctx.fillRect(0, 0, canvasW, canvasH);

  // Scan lines
  ctx.save();
  ctx.globalAlpha = 0.22;
  ctx.fillStyle = "rgba(255,255,255,0.035)";
  const scanOffset = (elapsed * 28) % 36;
  for (let sy = -scanOffset; sy < canvasH; sy += 36) {
    ctx.fillRect(0, sy, canvasW, 1);
  }
  ctx.restore();

  // Draw dust particles
  for (let i = 0; i < dust.length; i++) {
    const d = dust[i];
    d.x += d.vx;
    d.y += d.vy;
    d.p += 0.018;
    if (d.x < -10) d.x = canvasW + 10;
    if (d.x > canvasW + 10) d.x = -10;
    if (d.y < -10) d.y = canvasH + 10;
    if (d.y > canvasH + 10) d.y = -10;
    const alpha = d.a * (0.58 + Math.sin(d.p + elapsed * 0.8) * 0.34);
    ctx.beginPath();
    ctx.arc(d.x, d.y, d.r, 0, Math.PI * 2);
    ctx.fillStyle = `rgba(255,255,255,${Math.max(0, alpha)})`;
    ctx.fill();
  }

  // Draw streaks with additive blending
  ctx.save();
  ctx.globalCompositeOperation = "lighter";
  for (let k = 0; k < streaks.length; k++) {
    const st = streaks[k];
    const travel =
      ((elapsed * st.speed * 240 + st.x + Math.sin(elapsed * 0.8 + st.phase) * 28) %
        (canvasW + st.len + 180));
    const px = travel - st.len - 90;
    const py = st.y + Math.sin(elapsed * 0.75 + st.phase) * 18;
    const fade =
      smoothstep(st.delay * 0.55, st.delay * 0.55 + 0.52, elapsed) *
      (1 - smoothstep(3.52, 4.12, elapsed));
    if (fade <= 0) continue;

    ctx.save();
    ctx.translate(px, py);
    ctx.rotate(st.angle);
    const sg = ctx.createLinearGradient(-st.len * 0.5, 0, st.len * 0.5, 0);
    sg.addColorStop(0, st.color + "0)");
    sg.addColorStop(0.52, st.color + (st.alpha * fade).toFixed(3) + ")");
    sg.addColorStop(1, "rgba(255,255,255,0)");
    ctx.strokeStyle = sg;
    ctx.lineWidth = st.width;
    ctx.shadowColor = st.color + (0.34 * fade).toFixed(3) + ")";
    ctx.shadowBlur = 18;
    ctx.beginPath();
    ctx.moveTo(-st.len * 0.5, 0);
    ctx.lineTo(st.len * 0.5, 0);
    ctx.stroke();
    ctx.restore();
  }

  // Center line/slit
  const lineT = easeOutCubic((elapsed - 0.12) / 1.18);
  const exitFade = 1 - smoothstep(3.58, 4.12, elapsed);
  if (lineT > 0 && exitFade > 0) {
    const centerY = canvasH * 0.5 + Math.sin(elapsed * 1.4) * 1.6;
    const slitW = canvasW * (0.16 + lineT * 0.72);
    const left = canvasW * 0.5 - slitW * 0.5;
    const right = canvasW * 0.5 + slitW * 0.5;
    const coreAlpha = (0.34 + lineT * 0.58) * exitFade;

    // Main slit gradient
    const slitGrad = ctx.createLinearGradient(left, centerY, right, centerY);
    slitGrad.addColorStop(0, "rgba(255,83,103,0)");
    slitGrad.addColorStop(0.18, `rgba(255,83,103,${(0.18 * exitFade).toFixed(3)})`);
    slitGrad.addColorStop(0.5, `rgba(255,255,255,${coreAlpha.toFixed(3)})`);
    slitGrad.addColorStop(0.68, `rgba(244,210,138,${(0.38 * exitFade).toFixed(3)})`);
    slitGrad.addColorStop(0.84, `rgba(122,215,194,${(0.2 * exitFade).toFixed(3)})`);
    slitGrad.addColorStop(1, "rgba(122,215,194,0)");

    ctx.shadowColor = `rgba(244,210,138,${(0.48 * exitFade).toFixed(3)})`;
    ctx.shadowBlur = 42 + lineT * 42;
    ctx.lineCap = "round";
    ctx.strokeStyle = slitGrad;
    ctx.lineWidth = 1.4 + lineT * 2.2;
    ctx.beginPath();
    ctx.moveTo(left, centerY);
    ctx.lineTo(right, centerY);
    ctx.stroke();

    // Ignition flash
    const ignition = Math.exp(-Math.pow((elapsed - 0.72) / 0.26, 2));
    if (ignition > 0.018) {
      const ig = ctx.createLinearGradient(0, centerY, canvasW, centerY);
      ig.addColorStop(0, "rgba(122,215,194,0)");
      ig.addColorStop(0.46, `rgba(122,215,194,${(0.07 * ignition).toFixed(3)})`);
      ig.addColorStop(0.5, `rgba(255,255,255,${(0.16 * ignition).toFixed(3)})`);
      ig.addColorStop(0.54, `rgba(255,83,103,${(0.08 * ignition).toFixed(3)})`);
      ig.addColorStop(1, "rgba(244,210,138,0)");
      ctx.fillStyle = ig;
      ctx.fillRect(0, centerY - 48 * ignition, canvasW, 96 * ignition);
    }

    // Wave effect
    const waveAlpha = smoothstep(0.72, 1.95, elapsed) * exitFade;
    if (waveAlpha > 0) {
      ctx.shadowBlur = 20;
      ctx.strokeStyle = `rgba(244,210,138,${(0.22 * waveAlpha).toFixed(3)})`;
      ctx.lineWidth = 1;
      ctx.beginPath();
      const steps = 82;
      for (let wi = 0; wi <= steps; wi++) {
        const u = wi / steps;
        const x = left + slitW * u;
        const edge = 1 - Math.abs(u - 0.5) * 2;
        const amp = (4 + 18 * lineT) * Math.pow(Math.max(0, edge), 1.4) * waveAlpha;
        const y =
          centerY +
          Math.sin(u * 34 + elapsed * 8.2) * amp +
          Math.sin(u * 87 - elapsed * 5.1) * amp * 0.18;
        if (wi === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.stroke();
    }

    // Shards
    const shardT = smoothstep(0.72, 2.45, elapsed) * exitFade;
    for (let si = 0; si < shards.length; si++) {
      const sh = shards[si];
      const drift = Math.sin(elapsed * 1.7 + sh.phase) * 22;
      const sx = canvasW * 0.5 + sh.ox * (0.18 + shardT * 0.82) + drift;
      const sy2 = centerY + sh.oy * (0.2 + shardT * 0.92);
      const localAlpha =
        sh.alpha * shardT * (0.62 + Math.sin(elapsed * 5 + sh.phase) * 0.38);
      if (localAlpha <= 0) continue;

      ctx.save();
      ctx.translate(sx, sy2);
      ctx.rotate(((-6 + sh.skew * 0.1) * Math.PI) / 180);
      ctx.fillStyle = sh.color + Math.max(0, localAlpha).toFixed(3) + ")";
      ctx.shadowColor =
        sh.color + Math.min(0.38, localAlpha * 1.2).toFixed(3) + ")";
      ctx.shadowBlur = 14;
      ctx.beginPath();
      ctx.moveTo(-sh.w * 0.5, -sh.h * 0.5);
      ctx.lineTo(sh.w * 0.5, -sh.h * 0.5);
      ctx.lineTo(sh.w * 0.5 + sh.skew, sh.h * 0.5);
      ctx.lineTo(-sh.w * 0.5 + sh.skew, sh.h * 0.5);
      ctx.closePath();
      ctx.fill();
      ctx.restore();
    }

    // Flash effect
    const flash = Math.exp(-Math.pow((elapsed - 2.52) / 0.38, 2));
    if (flash > 0.015) {
      const fg = ctx.createLinearGradient(0, centerY, canvasW, centerY);
      fg.addColorStop(0, "rgba(255,83,103,0)");
      fg.addColorStop(0.48, `rgba(255,255,255,${(0.2 * flash).toFixed(3)})`);
      fg.addColorStop(0.52, `rgba(244,210,138,${(0.24 * flash).toFixed(3)})`);
      fg.addColorStop(1, "rgba(122,215,194,0)");
      ctx.fillStyle = fg;
      ctx.fillRect(0, centerY - 46 * flash, canvasW, 92 * flash);
    }
  }

  ctx.restore();

  animationId = requestAnimationFrame(() => animate(canvas, ctx));
}

onMounted(() => {
  if (canvasRef.value) {
    initCanvas(canvasRef.value);
  }
  setTimeout(() => {
    splashState.value = "ready";
  }, window.matchMedia("(prefers-reduced-motion: reduce)").matches ? 900 : 5000);
});

onUnmounted(() => {
  if (animationId) {
    cancelAnimationFrame(animationId);
  }
  if (resizeHandler) window.removeEventListener("resize", resizeHandler);
});

function handleEnter() {
  if (splashState.value !== "ready") return;
  splashState.value = "exiting";
  setTimeout(() => {
    emit("enter");
  }, 1200);
}
</script>

<template>
  <div id="splash" :class="splashState" @click="handleEnter">
    <canvas ref="canvasRef" id="splash-canvas"></canvas>
    <div class="splash-bg-noise"></div>

    <div class="splash-content">
      <div class="splash-wordmark" aria-label="悦音">
        <span class="splash-word-yue">悦</span>
        <span class="splash-word-yin">音</span>
      </div>
      <div class="splash-signal-line"></div>
      <div class="splash-sub">沉浸式音乐播放器</div>
      <div class="splash-enter" aria-hidden="true">点击进入</div>
    </div>
  </div>
</template>

<style scoped>
#splash {
  position: fixed;
  inset: 0;
  z-index: 300;
  background: #010304;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: auto;
  opacity: 1;
  overflow: hidden;
  transition: opacity 1180ms cubic-bezier(0.16, 1, 0.3, 1),
    transform 1180ms cubic-bezier(0.16, 1, 0.3, 1);
  box-shadow: inset 0 0 180px rgba(0, 0, 0, 0.88);
}

#splash::before {
  content: "";
  position: absolute;
  inset: -8%;
  z-index: 0;
  background: linear-gradient(
      115deg,
      transparent 0%,
      rgba(255, 83, 103, 0.055) 24%,
      transparent 42%,
      rgba(244, 210, 138, 0.052) 62%,
      transparent 82%
    ),
    repeating-linear-gradient(
      90deg,
      rgba(255, 255, 255, 0.03) 0 1px,
      transparent 1px 54px
    ),
    repeating-linear-gradient(
      0deg,
      rgba(255, 255, 255, 0.02) 0 1px,
      transparent 1px 46px
    ),
    linear-gradient(180deg, #020606 0%, #050607 42%, #000 100%);
  filter: blur(0.4px);
  opacity: 0.9;
  animation: splash-field-breathe 7s ease-in-out infinite alternate;
  pointer-events: none;
}

#splash::after {
  content: "";
  position: absolute;
  inset: 0;
  z-index: 2;
  background: linear-gradient(
      90deg,
      rgba(0, 0, 0, 0.82),
      transparent 21%,
      transparent 79%,
      rgba(0, 0, 0, 0.82)
    ),
    linear-gradient(
      180deg,
      rgba(0, 0, 0, 0.68),
      transparent 32%,
      transparent 64%,
      rgba(0, 0, 0, 0.74)
    );
  pointer-events: none;
}

#splash.ready {
  cursor: pointer;
}

#splash.exiting {
  pointer-events: none;
  opacity: 0;
  transform: scale(1.018);
}

#splash-canvas {
  position: absolute;
  inset: 0;
  z-index: 1;
  opacity: 1;
  transition: opacity 1100ms cubic-bezier(0.22, 1, 0.36, 1),
    transform 1100ms cubic-bezier(0.22, 1, 0.36, 1);
}

#splash.exiting #splash-canvas {
  opacity: 0.3;
  transform: scale(1.012);
}

.splash-bg-noise {
  position: absolute;
  inset: 0;
  z-index: 3;
  opacity: 0.038;
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 180 180' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='.9' numOctaves='2' stitchTiles='stitch'/%3E%3CfeColorMatrix type='saturate' values='0'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)' opacity='.55'/%3E%3C/svg%3E");
  background-size: 180px 180px;
  mix-blend-mode: screen;
  pointer-events: none;
}

.splash-content {
  position: relative;
  z-index: 10;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  pointer-events: none;
  transform: translateY(4px);
  transition: opacity 680ms cubic-bezier(0.22, 1, 0.36, 1),
    transform 860ms cubic-bezier(0.22, 1, 0.36, 1);
}

.splash-wordmark {
  position: relative;
  display: flex;
  align-items: baseline;
  justify-content: center;
  height: clamp(70px, 12vw, 136px);
  min-width: min(74vw, 760px);
  font-size: clamp(52px, 8.8vw, 112px);
  line-height: 0.92;
  font-weight: 720;
  letter-spacing: -0.055em;
  color: #f8f8f2;
  text-shadow: 0 20px 82px rgba(0, 0, 0, 0.68),
    -2px 0 18px rgba(255, 83, 103, 0.16), 2px 0 18px rgba(122, 215, 194, 0.12);
  isolation: isolate;
  filter: drop-shadow(0 0 22px rgba(244, 210, 138, 0.075));
}

.splash-word-yue,
.splash-word-yin {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  white-space: nowrap;
  will-change: opacity, transform, letter-spacing;
}

.splash-word-yue {
  opacity: 0;
  animation: splash-yue-in 5200ms cubic-bezier(0.22, 1, 0.36, 1) forwards;
  text-shadow: -2px 0 0 rgba(255, 83, 103, 0.24),
    2px 0 0 rgba(122, 215, 194, 0.18), 0 22px 72px rgba(0, 0, 0, 0.58),
    0 0 34px rgba(244, 210, 138, 0.1);
}

.splash-word-yin {
  opacity: 0;
  letter-spacing: -0.018em;
  background: linear-gradient(
    94deg,
    rgba(255, 255, 255, 0.06),
    #fff 26%,
    rgba(244, 210, 138, 0.98) 48%,
    rgba(122, 215, 194, 0.9) 68%,
    rgba(255, 255, 255, 0.82)
  );
  background-size: 300% 100%;
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  -webkit-text-fill-color: transparent;
  animation: splash-yin-in 5200ms cubic-bezier(0.22, 1, 0.36, 1) forwards;
}

.splash-signal-line {
  position: relative;
  width: min(460px, 54vw);
  height: 2px;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(122, 215, 194, 0.22),
    rgba(255, 255, 255, 0.78),
    rgba(244, 210, 138, 0.66),
    rgba(255, 83, 103, 0.22),
    transparent
  );
  opacity: 0;
  transform: scaleX(0.12);
  box-shadow: 0 0 18px rgba(244, 210, 138, 0.24),
    0 0 34px rgba(122, 215, 194, 0.1);
  animation: splash-signal-line 4200ms cubic-bezier(0.22, 1, 0.36, 1) forwards;
}

.splash-signal-line::after {
  content: "";
  position: absolute;
  left: 50%;
  top: 50%;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.82);
  box-shadow: 0 0 24px rgba(244, 210, 138, 0.54);
  transform: translate(-50%, -50%) scale(0.32);
  opacity: 0;
  animation: splash-signal-blip 4200ms cubic-bezier(0.22, 1, 0.36, 1) forwards;
}

.splash-sub {
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.22em;
  color: rgba(255, 255, 255, 0.34);
  text-transform: uppercase;
  opacity: 0;
  animation: splash-sub-in 4200ms cubic-bezier(0.22, 1, 0.36, 1) forwards;
}

.splash-enter {
  margin-top: 8px;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.24em;
  color: rgba(255, 255, 255, 0.62);
  text-transform: uppercase;
  opacity: 0;
  transform: translateY(10px);
  text-shadow: 0 0 18px rgba(244, 210, 138, 0.24),
    0 0 34px rgba(122, 215, 194, 0.12);
  transition: opacity 620ms cubic-bezier(0.22, 1, 0.36, 1),
    transform 620ms cubic-bezier(0.22, 1, 0.36, 1);
}

#splash.ready .splash-enter {
  opacity: 1;
  transform: translateY(0);
  animation: splash-enter-pulse 1800ms ease-in-out infinite alternate;
}

@keyframes splash-field-breathe {
  0% {
    opacity: 0.72;
    transform: scale(1);
  }
  100% {
    opacity: 1;
    transform: scale(1.035);
  }
}

@keyframes splash-yue-in {
  0% {
    opacity: 0;
    clip-path: inset(48% 0 49% 0);
    transform: translate(calc(-50% - 10px), -42%) skewX(-10deg) scaleX(1.08);
    letter-spacing: 0.055em;
  }
  14% {
    opacity: 0.92;
    clip-path: inset(40% 0 42% 0);
    transform: translate(calc(-50% - 4px), -50%) skewX(-4deg) scaleX(1.04);
    letter-spacing: 0.014em;
  }
  26% {
    opacity: 1;
    clip-path: inset(0);
    transform: translate(-50%, -50%) skewX(0) scaleX(1);
    letter-spacing: -0.04em;
  }
  48% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1);
  }
  67% {
    opacity: 1;
    transform: translate(calc(-50% - clamp(66px, 10.8vw, 130px)), -50%)
      scale(0.998);
    letter-spacing: -0.055em;
  }
  100% {
    opacity: 1;
    transform: translate(calc(-50% - clamp(66px, 10.8vw, 130px)), -50%)
      scale(0.998);
  }
}

@keyframes splash-yin-in {
  0%,
  32% {
    opacity: 0;
    clip-path: inset(52% 0 44% 0);
    transform: translate(calc(-50% + clamp(78px, 12vw, 142px)), -50%)
      skewX(9deg) scaleX(1.06);
    background-position: 0 0;
  }
  48% {
    opacity: 0.88;
    clip-path: inset(34% 0 32% 0);
    transform: translate(calc(-50% + clamp(72px, 11.5vw, 138px)), -50%)
      skewX(3deg) scaleX(1.02);
    background-position: 52% 0;
  }
  66% {
    opacity: 1;
    clip-path: inset(0);
    transform: translate(calc(-50% + clamp(70px, 11.4vw, 136px)), -50%) scale(1);
    background-position: 76% 0;
  }
  100% {
    opacity: 1;
    transform: translate(calc(-50% + clamp(70px, 11.4vw, 136px)), -50%) scale(1);
    background-position: 100% 0;
  }
}

@keyframes splash-signal-line {
  0%,
  28% {
    opacity: 0;
    transform: scaleX(0.1);
  }
  44% {
    opacity: 0.98;
    transform: scaleX(1.05);
  }
  64% {
    opacity: 0.7;
    transform: scaleX(0.82);
  }
  76% {
    opacity: 1;
    transform: scaleX(1.14);
    box-shadow: 0 0 28px rgba(244, 210, 138, 0.36),
      0 0 60px rgba(122, 215, 194, 0.18);
  }
  100% {
    opacity: 0.3;
    transform: scaleX(0.64);
  }
}

@keyframes splash-signal-blip {
  0%,
  42% {
    opacity: 0;
    left: 18%;
    transform: translate(-50%, -50%) scale(0.24);
  }
  62% {
    opacity: 0.94;
    left: 50%;
    transform: translate(-50%, -50%) scale(1);
  }
  76% {
    opacity: 1;
    left: 50%;
    transform: translate(-50%, -50%) scale(1.45);
  }
  100% {
    opacity: 0.16;
    left: 82%;
    transform: translate(-50%, -50%) scale(0.46);
  }
}

@keyframes splash-sub-in {
  0%,
  38% {
    opacity: 0;
    transform: translateY(7px);
  }
  56% {
    opacity: 0.58;
    transform: translateY(0);
  }
  100% {
    opacity: 0.42;
    transform: translateY(0);
  }
}

@keyframes splash-enter-pulse {
  0% {
    opacity: 0.46;
    text-shadow: 0 0 14px rgba(244, 210, 138, 0.16),
      0 0 26px rgba(122, 215, 194, 0.08);
  }
  100% {
    opacity: 0.78;
    text-shadow: 0 0 22px rgba(244, 210, 138, 0.3),
      0 0 42px rgba(122, 215, 194, 0.16);
  }
}
</style>
