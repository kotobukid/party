<script setup lang="ts">
import {onMounted, ref} from "vue";

import EventList from "./components/EventList.vue";
import AppConfiguration from "./components/AppConfiguration.vue";

let show_config = ref<boolean>(false);
const show_limit = ref<0 | 1 | 2>(0);
const regular_wp = ref<0 | 1 | 2>(0);
const format = ref<0 | 1 | 2 | 3>(0);

document.addEventListener('keydown', (event) => {
  if (event.key === 'F1') {
    show_config.value = !show_config.value;
    event.preventDefault(); // ブラウザのデフォルトの挙動をキャンセル
  }
});

onMounted(() => {
  const defaultLimit = 0;
  const defaultRegularWp = 0;
  const defaultFormat = 0;

  let temp = localStorage.getItem('wx-party.limit');
  let parsedLimit = temp ? parseInt(temp) : null;
  // @ts-ignore
  show_limit.value = (parsedLimit !== null && [0, 1, 2].includes(parsedLimit)) ? parsedLimit : defaultLimit;

  temp = localStorage.getItem('wx-party.regular-wp');
  let parsedRegularWp = temp ? parseInt(temp) : null;
  // @ts-ignore
  regular_wp.value = (parsedRegularWp !== null && [0, 1, 2].includes(parsedRegularWp)) ? parsedRegularWp : defaultRegularWp;

  temp = localStorage.getItem('wx-party.format');
  let parsedFormat = temp ? parseInt(temp) : null;
  // @ts-ignore
  format.value = (parsedFormat !== null && [0, 1, 2, 3].includes(parsedFormat)) ? parsedFormat : defaultFormat;
});

const limit_changed = (limit: 0 | 1 | 2) => {
  localStorage.setItem("wx-party.limit", limit.toString());

  show_limit.value = limit;
};

const regular_wp_changed = (wp: 0 | 1 | 2) => {
  localStorage.setItem("wx-party.regular-wp", wp.toString());
  regular_wp.value = wp;
};

const format_changed = (f: 0 | 1 | 2 | 3) => {
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
