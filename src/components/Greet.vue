<script setup lang="ts">
import {onMounted, ref, type Ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const greetMsg: Ref<EventDetail[]> = ref([]);

type EventDetail = {
    name: string,
    con: string,
    shop_name: string,
    shop_link: string,
    time: string,
    format: string,
}

onMounted(async () => {
    const events: EventDetail[] = await invoke("greet");
    greetMsg.value = events.sort((event: EventDetail, event_right: EventDetail) => {
        return event.time.localeCompare(event_right.time);
    });
});

</script>

<template>
<table>
    <colgroup>
        <col style="width: 200px;"/>
        <col style="width: 170px;"/>
        <col style="width: 300px;"/>
        <col style="width: 345px;"/>
    </colgroup>
    <thead>
    <tr>
        <th>フォーマット</th>
        <th>時刻</th>
        <th>店名</th>
        <th>イベント名</th>
    </tr>
    </thead>
    <tbody>
    <tr v-for="e in greetMsg">
        <td :data-format="e.format">{{ e.format }}</td>
        <td>{{ e.time }}</td>
        <td>
            <span class="con">{{ e.con }}</span>
            <a target="_blank" :href="e.shop_link">{{ e.shop_name }}</a>
        </td>
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

tr:nth-child(2n) {
    background-color: #c6c6c6;
}

a {
    color: darkblue;
    font-weight: bolder;
}

span.con {
    display: inline-block;
    width: 4rem;
    text-align: center;
    border: 1px solid grey;
    background-color: white;
    border-radius: 5px;
    padding: 5px;
    margin-right: 5px;
    color: black;
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