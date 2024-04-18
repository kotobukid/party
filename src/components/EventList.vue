<script setup lang="ts">
import {computed, onMounted, ref, type Ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {
  EventFormatAllStar,
  EventFormatKeySelection,
  EventFormatDivaSelection,
  type EVENT_FORMAT, EVENT_TYPE, PartyTypeAny, PartyTypeRegular
} from "../const.ts";

const props = defineProps<{
  show_limit: number,
  regular_wp: EVENT_TYPE,
  format: EVENT_FORMAT
}>()

type EventDetail = {
  name: string,
  state: string,
  shop: string,
  url: string,
  time_s: string,
  date: Date,
  category: string,
  format: string,
  is_regular_wp: boolean
};

type EventDetailOnServer = Omit<EventDetail, "is_regular_wp">

const load_first: Ref<boolean> = ref(false);

const events_all: Ref<EventDetail[]> = ref([]);

onMounted(async () => {
  const events: EventDetailOnServer[] = await invoke("fetch_events");

  events_all.value = events.map(e => {
    return {
      ...e,
      is_regular_wp: e.name.startsWith("WIXOSS PARTY")
    };
  });
  load_first.value = true;
});

const check_event_type = (filter: EVENT_TYPE, e: EventDetail): boolean => {
  if (filter === PartyTypeAny) {
    return true;
  } else if (filter === PartyTypeRegular) {
    return e.is_regular_wp;
  } else {
    return !e.is_regular_wp;
  }
};

const check_format = (filter: EVENT_FORMAT, e: EventDetail): boolean => {
  if (filter === 0) {
    return true;
  } else if (filter === EventFormatAllStar) {
    return e.format === 'オールスター';
  } else if (filter === EventFormatKeySelection) {
    return e.format === 'キーセレクション';
  } else if (filter === EventFormatDivaSelection) {
    return e.format === 'ディーヴァセレクション';
  } else {
    return e.category === 'ガチばとる' || e.category === '楽しくばとる';
  }
}

const events_to_show = computed((): Record<string, EventDetail[]> => {
  const now = new Date(Date.UTC(new Date().getUTCFullYear(), new Date().getUTCMonth(), new Date().getUTCDate()));
  const limit = props.show_limit === 0 ? 62 : props.show_limit;
  const XDaysLater = new Date(Date.UTC(now.getUTCFullYear(), now.getUTCMonth(), now.getUTCDate() + limit));

  const filteredAndSortedEvents = events_all.value
      .filter(event => {
        // イベントの日付をUTCで解析します
        const eventDate = new Date(Date.UTC(new Date(event.date).getUTCFullYear(), new Date(event.date).getUTCMonth(), new Date(event.date).getUTCDate()));
        return (eventDate >= now && eventDate <= XDaysLater)
            && check_event_type(props.regular_wp, event)
            && check_format(props.format, event)
      })
      .sort((a, b) => {
        const dateA = new Date(Date.UTC(new Date(a.date).getUTCFullYear(), new Date(a.date).getUTCMonth(), new Date(a.date).getUTCDate()));
        const dateB = new Date(Date.UTC(new Date(b.date).getUTCFullYear(), new Date(b.date).getUTCMonth(), new Date(b.date).getUTCDate()));
        return dateA.getTime() - dateB.getTime();
      });

  // イベントを年月日でグルーピング
  const groupedEvents = filteredAndSortedEvents.reduce((groups, event) => {
    const dateKey = new Date(Date.UTC(new Date(event.date).getUTCFullYear(), new Date(event.date).getUTCMonth(), new Date(event.date).getUTCDate())).toISOString().split('T')[0];

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

const no_events = computed(() => {
  return Object.keys(events_to_show.value).length === 0;
});
</script>

<template>
  <table>
    <colgroup>
      <col style="width: 222px;"/>
      <col style="width: 210px;"/>
      <col style="width: 370px;"/>
      <col style="width: 371px;"/>
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
    <tr class="event" v-for="event in events" :key="`${event.time_s}_${event.shop}_${event.name}`">
      <td :data-format="event.format">
        <span :data-category="event.category"></span>
        {{ event.format }}
      </td>
      <td>{{ event.time_s }}</td>
      <td>
        <a target="_blank" :href="event.url">{{ event.shop }}</a>
        <span class="con">{{ event.state }}</span>
      </td>
      <td>{{ event.name }}</td>
    </tr>
    </tbody>
    <tbody v-if="no_events && !load_first">
    <tr>
      <td colspan="4">読み込み中です</td>
    </tr>
    </tbody>
    <tbody v-if="no_events && load_first">
    <tr>
      <td colspan="4">イベントがありません</td>
    </tr>
    </tbody>
  </table>
</template>

<style scoped>
table {
  width: 1173px;
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

tr.event:hover td {
  background-color: lightgreen;
}

tr:nth-child(2n) {
  background-color: #c6c6c6;
}

a {
  color: #0000cb;
  font-weight: bolder;
}

span.con {
  margin-left: 5px;
  color: black;

  &:before {
    content: '(';
  }

  &:after {
    content: ')';
  }
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

td[data-format="ガチばとる"] {
  background-color: #103300;
  color: white;
  border-color: white;
}

td[data-format="楽しくばとる"] {
  background-color: #4bfa8c;
}


span[data-category=""] {
  &:after {
  }
}

span[data-category="ガチばとる"] {
  &:after {
    display: inline-block;
    width: 24px;
    height: 24px;
    content: url('/public/gachi.png');
    outline: 1px solid black;
    padding: 0;
    background-color: red;
    margin: 0;
    position: relative;
    top: 4px;
  }
}

span[data-category="楽しくばとる"] {
  &:after {
    display: inline-block;
    width: 24px;
    height: 24px;
    content: url('/public/tanoshiku.png');
    outline: 1px solid black;
    padding: 0;
    background-color: red;
    margin: 0;
    position: relative;
    top: 4px;
  }
}
</style>