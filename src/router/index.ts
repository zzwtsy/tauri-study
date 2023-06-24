import { RouteRecordRaw, createRouter, createWebHashHistory } from "vue-router";
import HomeView from "../components/HomeView.vue";
import DataView from "../components/DataView.vue";
import SettingView from "../components/SettingView.vue";

const routes: RouteRecordRaw[] = [
  { path: "/", redirect: "/home" },
  { path: "/home", name: "home", component: HomeView },
  { path: "/data", name: "data", component: DataView },
  { path: "/setting", name: "setting", component: SettingView },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});

export default router;
