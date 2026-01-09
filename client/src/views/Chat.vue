<template>
    <div class="container">
        <div class="header">
            <img :src="chatBubble" class="icon" />
            <p>Real-time-chat</p>
            <span>X</span>
        </div>

        <div class="menu-bar">
            <span>File</span>
            <span>Edit</span>
            <span>View</span>
            <span>Tools</span>
            <span>Help</span>
        </div>

        <div class="container-interior">
            <div class="container-esquerda">
                <div class="container-chat">
                    <p>asdasd</p>
                    <p>asdasd</p>
                </div>

                <div class="container-input">
                    <textarea
                        class="input-text"
                        placeholder="Digite sua mensagem..."
                        v-model="mensagem"
                        @keydown.enter.exact.prevent="enviarMensagem"
                    ></textarea>

                    <div class="btn-enviar" v-on:click="enviarMensagem">
                        Enviar
                    </div>
                </div>
            </div>

            <div class="container-direita">
                <div class="container-usuario-online">
                    <img :src="chatBubble" class="icon" />
                    <p>Usuario</p>
                </div>
                <div class="container-usuario-online">
                    <img :src="chatBubble" class="icon" />
                    <p>Usuario</p>
                </div>
                <div class="container-usuario-online">
                    <img :src="chatBubble" class="icon" />
                    <p>Usuario</p>
                </div>
                <div class="container-usuario-online">
                    <img :src="chatBubble" class="icon" />
                    <p>Usuario</p>
                </div>
            </div>
        </div>

    </div>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background-color: rgb(192, 192, 192);
    font-family: "PxPlus IBM VGA8", monospace;
    font-size: 14px;
    padding: 10px;
    box-sizing: border-box;
}

.header {
    display: flex;
    align-items: center;
    height: 35px;
    background-color: rgb(0, 0, 128);
    font-size: 20px;
}

.icon {
    width: 30px;
    height: 30px;
    margin-left: 8px;
    margin-right: 8px;
}

.header p {
    color: white;
}

.header span {
    margin-left: auto;
    margin-right: 8px;
    background-color: rgb(192, 192, 192);
    padding: 1px 6px;
    font-size: 16px;
    cursor: pointer;

    border-top: 1px solid white;
    border-left: 1px solid white;
    border-right: 1px solid rgb(128, 128, 128);
    border-bottom: 1px solid rgb(128, 128, 128);
}

.header span:active {
    border-top: 1px solid rgb(128, 128, 128);
    border-left: 1px solid rgb(128, 128, 128);
    border-right: 1px solid white;
    border-bottom: 1px solid white;
}

.menu-bar {
    margin-top: 10px;
    display: flex;
    height: 24px;
    align-items: center;
    font-size: 20px;

    background-color: rgb(192, 192, 192);

    border-bottom: 1px solid white;
    box-shadow: inset 0 -2px 0 rgb(128, 128, 128);
}

.menu-bar span {
    margin-left: 12px;
    padding: 2px 6px;
    cursor: default;
    user-select: none;
}

.menu-bar span:hover {
    background-color: rgb(0, 0, 128);
    color: white;
}

.container-interior {
    display: flex;
    flex: 1;
    margin-top: 10px;
    gap: 10px;
    min-height: 0;
}

.container-esquerda {
    flex: 0 0 75%;
    background-color: white;
    display: flex;
    flex-direction: column;
    min-height: 0;
    padding: 4px;

    background-color: rgb(192, 192, 192);

    border-top: 1px solid white;
    border-left: 1px solid white;
    border-right: 1px solid rgb(128, 128, 128);
    border-bottom: 1px solid rgb(128, 128, 128);
}

.container-chat {
    flex: 3;
    background-color: white;
    margin-bottom: 6px;
    padding: 6px;
    overflow-y: auto;
    min-height: 0;

    border-top: 3px solid rgb(0, 0, 0);
    border-left: 3px solid rgb(0, 0, 0);
    border-right: 3px solid white;
    border-bottom: 3px solid white;

    scrollbar-width: thin;                 /* Firefox */
    scrollbar-color: rgb(128,128,128) rgb(192,192,192);

    line-break: anywhere;
}

.container-chat::-webkit-scrollbar {
    width: 16px;
}

.container-chat::-webkit-scrollbar-track {
    background: rgb(192, 192, 192);
    border-left: 2px solid white;
}

.container-chat::-webkit-scrollbar-thumb {
    background: rgb(192, 192, 192);

    border-top: 2px solid white;
    border-left: 2px solid white;
    border-right: 2px solid rgb(128,128,128);
    border-bottom: 2px solid rgb(128,128,128);
}

.container-chat::-webkit-scrollbar-thumb:active {
    border-top: 2px solid rgb(128,128,128);
    border-left: 2px solid rgb(128,128,128);
    border-right: 2px solid white;
    border-bottom: 2px solid white;
}

.container-chat p {
    font-size: 20px;
    margin: 2px;
}

.container-input {
    flex: 1;
    display: flex;
    gap: 6px;
    padding: 6px;

    background-color: rgb(192, 192, 192);

    border-top: 2px solid rgb(0, 0, 0);
    border-left: 2px solid rgb(0, 0, 0);
    border-right: 2px solid rgb(192, 192, 192);
    border-bottom: 2px solid rgb(192, 192, 192);
}

.input-text {
    flex: 1;
    resize: none;
    outline: none;

    background-color: white;
    padding: 6px;

    font-family: "PxPlus IBM VGA8", monospace;
    font-size: 20px;
    line-height: 1.2;
    color: black;
    overflow-y: auto;

    border-top: 2px solid rgb(128, 128, 128);
    border-left: 2px solid rgb(128, 128, 128);
    border-right: 2px solid white;
    border-bottom: 2px solid white;
}

.btn-enviar {
    width: 120px;
    display: flex;
    align-items: center;
    justify-content: center;

    background-color: rgb(192, 192, 192);
    font-size: 22px;

    cursor: pointer;
    user-select: none;

    border-top: 2px solid white;
    border-left: 2px solid white;
    border-right: 2px solid rgb(128, 128, 128);
    border-bottom: 2px solid rgb(128, 128, 128);
}

.btn-enviar:active {
    border-top: 2px solid rgb(128, 128, 128);
    border-left: 2px solid rgb(128, 128, 128);
    border-right: 2px solid white;
    border-bottom: 2px solid white;
}


.container-direita {
    flex: 0 0 23%;
    padding: 4px;
    background-color: white;

    border-top: 2px solid black;
    border-left: 2px solid black;
    border-right: 2px solid rgb(102, 99, 99);
    border-bottom: 2px solid rgb(102, 99, 99);
}

.container-usuario-online {
    height: 55px;
    display: flex;
    align-items: center;
}

.container-usuario-online p {
    font-size: 22px;
    user-select: none;
}

.container-usuario-online:hover {
    background-color: rgb(0, 0, 128);
    color: white;
}

</style>

<script setup>
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import chatBubble from "@/assets/icons/chat-bubble.png";

const router = useRouter();
const idUsuario = ref("");
const nomeUsuario = ref("");
const corUsuario = ref("#000000");
const mensagem = ref("");
let socket = null;

onMounted(() => {
    idUsuario.value = localStorage.getItem("real-time-chat-id-usuario");
    nomeUsuario.value = localStorage.getItem("real-time-chat-nome-usuario");
    corUsuario.value = localStorage.getItem("real-time-chat-cor-usuario");

    if (!idUsuario.value || !nomeUsuario.value) {
        router.push("/");
    }

    socket = new WebSocket("ws://localhost:8080/api/chat");

    socket.onopen = () => {
        console.log("Conectado ao WebSocket");
    };

    socket.onmessage = (event) => {
        console.log("Mensagem recebida:", event.data);
    };

    socket.onerror = (err) => {
        console.error("Erro no WS", err);
    };
});

const enviarMensagem = () => {
    if (!mensagem.value.trim()) return;

    const objMensagem = {
        conteudo: mensagem.value,
        id_usuario_from: idUsuario.value,
        nome_usuario_from: nomeUsuario.value,
        cor_criacao: corUsuario.value
    };

    socket.send(JSON.stringify(objMensagem));

    mensagem.value = "";
}
</script>
