import '@/assets/index.css';
import '@tb-dev/vue-components/style';
import App from '@/App.vue';
import { createApp } from 'vue';
import { setCurrentApp } from '@tb-dev/vue';

const app = createApp(App);
setCurrentApp(app);

app.mount('#app');
