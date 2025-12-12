/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Plugins
import { registerPlugins } from "./plugins/index.ts";

// Components
//@ts-ignore
import App from "./App.vue";

// Composables
import { createApp } from "vue";
import "vuetify/styles";
// Styles
import "unfonts.css";

const app = createApp(App);

registerPlugins(app);

app.mount("#app");
