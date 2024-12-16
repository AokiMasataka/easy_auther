import App from 'vue';
import vuetify from './vuetify.ts';

export function registerPlugins (app: App) {
  app.use(vuetify)
}