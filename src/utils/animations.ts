import gsap from "gsap";

export function bindButtonAnimations(container: HTMLElement) {
  const buttons = container.querySelectorAll<HTMLElement>(".ctrl-btn");

  buttons.forEach((btn) => {
    const icon = btn.querySelector<HTMLElement>("svg");
    const isPlayBtn = btn.id === "play-btn";

    const enter = () => {
      gsap.to(btn, { y: -2, scale: isPlayBtn ? 1.04 : 1.07, duration: 0.2, ease: "power2.out" });
      if (icon) gsap.to(icon, { scale: 1.08, duration: 0.2, ease: "power2.out" });
    };

    const leave = () => {
      gsap.to(btn, { y: 0, scale: 1, rotate: 0, duration: 0.22, ease: "power2.out" });
      if (icon) gsap.to(icon, { scale: 1, rotate: 0, duration: 0.22, ease: "power2.out" });
    };

    const down = () => {
      gsap.to(btn, { scale: isPlayBtn ? 0.94 : 0.9, duration: 0.1, ease: "power2.out" });
      if (icon) gsap.to(icon, { scale: 0.88, duration: 0.1, ease: "power2.out" });
    };

    const up = () => {
      gsap.to(btn, { y: -2, scale: isPlayBtn ? 1.04 : 1.07, duration: 0.22, ease: "back.out(1.8)" });
      if (icon) gsap.to(icon, { scale: 1.06, duration: 0.22, ease: "back.out(1.8)" });
    };

    btn.addEventListener("pointerenter", enter);
    btn.addEventListener("pointerleave", leave);
    btn.addEventListener("pointerdown", down);
    btn.addEventListener("pointerup", up);
    btn.addEventListener("pointercancel", leave);

    // ponytail: store cleanup refs on element, no WeakMap overhead
    (btn as any)._gsapCleanup = () => {
      btn.removeEventListener("pointerenter", enter);
      btn.removeEventListener("pointerleave", leave);
      btn.removeEventListener("pointerdown", down);
      btn.removeEventListener("pointerup", up);
      btn.removeEventListener("pointercancel", leave);
      gsap.set(btn, { clearProps: "y,scale,rotate" });
      if (icon) gsap.set(icon, { clearProps: "scale,rotate" });
    };
  });
}

export function unbindButtonAnimations(container: HTMLElement) {
  container.querySelectorAll<HTMLElement>(".ctrl-btn").forEach((btn) => {
    (btn as any)._gsapCleanup?.();
  });
}

export function animateModeSwitch(btn: HTMLElement) {
  gsap.timeline({ defaults: { overwrite: true } })
    .fromTo(btn, { scale: 0.86, rotate: -8 }, { scale: 1.12, rotate: 4, duration: 0.16, ease: "power2.out" })
    .to(btn, { scale: 1, rotate: 0, duration: 0.34, ease: "back.out(2.1)" });
}

export function animateListItems(
  container: HTMLElement,
  selector: string,
  opts?: { limit?: number; stagger?: number }
) {
  const items = Array.from(container.querySelectorAll<HTMLElement>(selector))
    .slice(0, opts?.limit ?? 18);
  if (!items.length) return;

  gsap.fromTo(
    items,
    { autoAlpha: 0, y: 8, x: -6 },
    {
      autoAlpha: 1,
      y: 0,
      x: 0,
      duration: 0.22,
      stagger: opts?.stagger ?? 0.012,
      ease: "power2.out",
    }
  );
}
