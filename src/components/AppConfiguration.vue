<script setup lang="ts">
const emit = defineEmits([
    'limit-changed',
    'regular-wp-changed'
]);

const props = defineProps<{
  show_limit: number,
  regular_wp: 0 | 1 | 2
}>();

const show_limit_changed = (e: InputEvent) => {
  // @ts-ignore
  const limit = Number(e.target!.value!);

  emit('limit-changed', limit);
}

const regular_wp_changed = (e: InputEvent) => {
  // @ts-ignore
  const limit = Number(e.target!.value!);

  emit('regular-wp-changed', limit);
}
</script>

<template lang="pug">
.configuration
  label
    span 表示日数(0は全件)
    input(
      type="number"
      :value="props.show_limit"
      @change="show_limit_changed"
      min="0"
      max="31"
    )
  label
    span イベント種別
    select(:value="props.regular_wp" @change="regular_wp_changed")
      option(value="0") 指定なし
      option(value="1") 通常WP
      option(value="2") 特別イベント
</template>

<style scoped>
.configuration {
  border-radius: 4px;
  border: 1px solid grey;
  padding: 4px;
  margin-bottom: 5px;
}

label {
  margin-right: 10px;
}

input, select {
  margi-left: 4px;
  font-size: 1.2rem;
  padding: 6px;
  outline: 0 solid transparent;
  border: 1px solid #686868;
  border-radius: 4px;
  box-shadow: none;
}
</style>