import { createRouter } from 'vue-router'
import { createWebHistory } from 'vue-router'
import Login from '@/views/Login.vue'
import Chat from '@/views/Chat.vue'

const routes = [
    { path: '/', name: 'Login', component: Login },
    { path: '/chat', name: 'Chat', component: Chat },
]

export default createRouter({
    history: createWebHistory(),
    routes,
})
