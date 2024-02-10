<script setup lang="ts">
import {ref, type Ref} from "vue";

import EventList from "./components/EventList.vue";
import AppConfiguration from "./components/AppConfiguration.vue";

const show_config: Ref<boolean> = ref(false);
const show_limit: Ref<number> = ref(0);
const regular_wp: Ref<0 | 1 | 2> = ref(0);
const format: Ref<0 | 1 | 2 | 3> = ref(0);

document.addEventListener('keydown', (event) => {
  if (event.key === 'F1') {
    show_config.value = !show_config.value;
    event.preventDefault(); // ブラウザのデフォルトの挙動をキャンセル
  }
});
</script>

<template>
  <div class="container">
    <AppConfiguration
        v-if="show_config"
        :show_limit="show_limit"
        :regular_wp="regular_wp"
        :format="format"
        @limit-changed="(l: number) => {show_limit = l}"
        @regular-wp-changed="(b: 0 | 1 | 2) => {regular_wp = b}"
        @format-changed="(f: 0 | 1 | 2 | 3) => {format = f}"
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
