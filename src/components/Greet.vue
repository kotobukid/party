<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const greetMsg: Ref<EventDetail[]> = ref([]);

type EventDetail = {
    name: string,
    shop_name: string,
    shop_link: string,
    time: string,
    format: string,
}

onMounted(async () => {
    greetMsg.value = await invoke("greet");

})

</script>

<template>
<table>
    <colgroup>
        <col style="width: 150px;"/>
        <col style="width: 300px;"/>
        <col style="width: 200px;"/>
        <col style="width: 345px;"/>
    </colgroup>
    <thead>
    <tr>
        <th>時刻</th>
        <th>店名</th>
        <th>フォーマット</th>
        <th>イベント名</th>
    </tr>
    </thead>
    <tbody>
    <tr v-for="e in greetMsg">
        <td>{{ e.time }}</td>
        <td><a target="_blank" :href="e.shop_link">{{ e.shop_name }}</a></td>
        <td :data-format="e.format">{{ e.format }}</td>
        <td>{{ e.name }}</td>
    </tr>
    </tbody>
</table>
</template>

<style scoped>
table {
    width: 1170px;
    table-layout: fixed;
    border-collapse: collapse;
}

th, td {
    border: 1px solid black;
    padding: 5px;
}

th {
    color: white;
    background-color: #121212;
}

td {
    text-align: left;
}

tr:hover {
    background-color: lightgreen;
}

a {
    color: darkblue;
    font-weight: bolder;
}

td[data-format="オールスター"] {
    background-color: pink;
}
td[data-format="キーセレクション"] {
    background-color: orange;
}
td[data-format="ディーヴァセレクション"] {
    background-color: lightblue;
}
</style>