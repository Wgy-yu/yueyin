import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("../views/HomeView.vue"),
    },
    {
      path: "/player",
      name: "player",
      component: () => import("../views/PlayerView.vue"),
    },
  ],
});

export default router;
