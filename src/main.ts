import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus';

import 'element-plus/theme-chalk/dark/css-vars.css';
import 'element-plus/dist/index.css';
import './styles/global-style.css';

createApp(App)
  .use(ElementPlus)
  .mount("#app");
