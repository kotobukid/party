<script setup lang="ts">
import {onMounted, ref} from "vue";

import EventList from "./components/EventList.vue";
import AppConfiguration from "./components/AppConfiguration.vue";
import {
  type EVENT_FORMAT,
  type EVENT_TYPE,
  EventFormatAny,
  EventFormatAllStar,
  EventFormatKeySelection,
  EventFormatDivaSelection,
  EventFormatShiromado
} from "./const.ts";
import {PartyTypeAny, PartyTypeSpecial, PartyTypeRegular} from "./const.ts";

let show_config = ref<boolean>(false);
const show_limit = ref<number>(0);
const regular_wp = ref<EVENT_TYPE>(PartyTypeAny);
const format = ref<EVENT_FORMAT>(EventFormatAny);

document.addEventListener('keydown', (event) => {
  if (event.key === 'F1') {
    show_config.value = !show_config.value;
    event.preventDefault(); // ブラウザのデフォルトの挙動をキャンセル
  }
});

onMounted(() => {
  const defaultLimit: number = 0;
  const defaultRegularWp: EVENT_TYPE = PartyTypeAny;
  const defaultFormat: EVENT_FORMAT = EventFormatAny;

  let temp = localStorage.getItem('wx-party.limit');
  let parsedLimit = temp ? parseInt(temp) : null;
  // @ts-ignore
  show_limit.value = (parsedLimit !== null) ? parsedLimit : defaultLimit;

  temp = localStorage.getItem('wx-party.regular-wp');
  let parsedRegularWp = (temp ? parseInt(temp) : null) as EVENT_TYPE;
  // @ts-ignore
  regular_wp.value = (parsedRegularWp !== null && [
    PartyTypeAny,
    PartyTypeRegular,
    PartyTypeSpecial
  ].includes(parsedRegularWp)) ? parsedRegularWp : defaultRegularWp;

  temp = localStorage.getItem('wx-party.format');
  let parsedFormat = temp ? parseInt(temp) : null;
  // @ts-ignore
  format.value = (parsedFormat !== null && [
    EventFormatAny,
    EventFormatAllStar,
    EventFormatKeySelection,
    EventFormatDivaSelection,
    EventFormatShiromado
  ].includes(parsedFormat)) ? parsedFormat : defaultFormat;
});

const limit_changed = (limit: number) => {
  localStorage.setItem("wx-party.limit", limit.toString());

  show_limit.value = limit;
};

const regular_wp_changed = (wp: EVENT_TYPE) => {
  localStorage.setItem("wx-party.regular-wp", wp.toString());
  regular_wp.value = wp;
};

const format_changed = (f: EVENT_FORMAT) => {
  localStorage.setItem('wx-party.format', f.toString());
  format.value = f;
};
</script>

<template>
  <div class="container">
    <AppConfiguration
        v-if="show_config"
        :show_limit="show_limit"
        :regular_wp="regular_wp"
        :format="format"
        @limit-changed="limit_changed"
        @regular-wp-changed="regular_wp_changed"
        @format-changed="format_changed"
    ></AppConfiguration>
    <EventList
        :show_limit="show_limit"
        :regular_wp="regular_wp"
        :format="format"
    />
  </div>
</template>

<style scoped>
</style>
