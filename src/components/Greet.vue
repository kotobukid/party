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
        <col style="width: 600px;"/>
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
        <td>{{ e.format }}</td>
        <td>{{ e.name }}</td>
    </tr>
    </tbody>
</table>
</template>

<style scoped>
table {
    width: 1460px;
    table-layout: fixed;
    border-collapse: collapse;
}

th, td {
    border: 1px solid black;
    padding: 5px;
}

th {
    background-color: #121212;
}

td {
    text-align: left;
}
</style>