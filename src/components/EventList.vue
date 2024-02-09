<script setup lang="ts">
import {computed, onMounted, ref, type Ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const props = defineProps<{
  show_limit: number
}>()

console.log(props.show_limit);

type EventDetail = {
  name: string,
  con: string,
  shop_name: string,
  shop_link: string,
  time_s: string,
  datetime: Date,
  format: string,
};

const events_all: Ref<EventDetail[]> = ref([]);

onMounted(async () => {
  const events: EventDetail[] = await invoke("fetch_events");
  events_all.value = events;
});


const events_to_show = computed((): Record<string, EventDetail[]> => {
  const now = new Date();

  const limit = props.show_limit === 0 ? 30 : props.show_limit + 1;

  const threeDaysLater = new Date(now.getFullYear(), now.getMonth(), now.getDate() + limit);

  const filteredAndSortedEvents = events_all.value
      .filter(event => {
        const eventDate = new Date(event.datetime);
        return eventDate >= now && eventDate <= threeDaysLater;
      })
      .sort((a, b) => {
        const dateA = new Date(a.datetime);
        const dateB = new Date(b.datetime);
        return dateA.getTime() - dateB.getTime();
      });


  // イベントを年月日でグルーピング
  const groupedEvents = filteredAndSortedEvents.reduce((groups, event) => {
    const dateKey = new Date(event.datetime).toISOString().split('T')[0];

    // グループにこの日付がまだ存在しない場合は、空の配列を用意
    if (!groups[dateKey]) {
      groups[dateKey] = [];
    }

    // 当該日付のグループにイベントを追加
    groups[dateKey].push(event);

    return groups;
  }, {} as { [key: string]: EventDetail[] });
  return groupedEvents;
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
    <tbody v-for="(events, date) in events_to_show" :key="date">
    <tr class="date">
      <td colspan="4">{{ date }}</td>
    </tr>
    <tr v-for="event in events" :key="event.name">
      <td :data-format="event.format">{{ event.format }}</td>
      <td>{{ event.time_s }}</td>
      <td>
        <span class="con">{{ event.con }}</span>
        <a target="_blank" :href="event.shop_link">{{ event.shop_name }}</a>
      </td>
      <td>{{ event.name }}</td>
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

tr.date td {
  background-color: #868686;
  color: white;
  font-weight: bold;
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