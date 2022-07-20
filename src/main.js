import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { GlobalCmComponent } from "codemirror-editor-vue3";


const app = createApp(App);

app.use(GlobalCmComponent, { componentName: "codemirror" });
app.mount('#app')
