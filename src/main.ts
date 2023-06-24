import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import "@csstools/normalize.css";
import router from "./router";
import ElementPlus from "element-plus";
import zhCn from "element-plus/lib/locale/lang/zh-cn"

const app = createApp(App);
app.use(router);
app.use(ElementPlus, {
  locale: zhCn,
});
app.mount("#app");
