import { createApp } from "vue";
import "./style.css";
import "cesium/Source/Widgets/widgets.css";
import vuetify from "./plugins/vuetify";
import App from "./App.vue";


createApp(App).use(vuetify).mount("#app");
