<template>
    <div class="login-container">
        <h1>Entrar no Chat</h1>

        <input v-model="name" type="text" placeholder="Seu nome" />

        <button @click="enterChat">Entrar</button>
    </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const name = ref("");

async function enterChat() {
    if (name.value.trim().length === 0) {
        alert("Digite um nome!");
        return;
    }

    const nome = name.value;

    localStorage.setItem("username", name.value);

    const response = await fetch(`${import.meta.env.VITE_API_URL}/usuarios`, {
        method: 'POST',
        body: JSON.stringify({
            nome
        }),
        headers: {
            'Content-Type': 'application/json'
        }
    })

    router.push("/chat");
}
</script>

<style>
.login-container {
    display: flex;
    flex-direction: column;
    width: 300px;
    margin: 100px auto;
    gap: 10px;
}
</style>
