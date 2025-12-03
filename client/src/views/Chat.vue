<template>
    <div>
        <h2>Sala de Chat - Geral</h2>
        <p>Bem-vindo, {{ username }}!</p>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const username = ref("");

onMounted(() => {
    username.value = localStorage.getItem("username");

    if (!username.value) {
        router.push("/");
    }
});
</script>
