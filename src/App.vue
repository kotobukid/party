<script setup lang="ts">
import {ref, type Ref} from "vue";

import EventList from "./components/EventList.vue";
import AppConfiguration from "./components/AppConfiguration.vue";

const show_config: Ref<boolean> = ref(false);
const show_limit: Ref<number> = ref(0);

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
        @limit-changed="(l) => {show_limit = l}"
    ></AppConfiguration>
    <EventList :show_limit="show_limit"/>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
