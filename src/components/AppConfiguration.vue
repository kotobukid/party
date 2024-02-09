<script setup lang="ts">
const emit = defineEmits(['limit-changed']);

const props = defineProps<{
  show_limit: number
}>();

const show_limit_changed = (e: InputEvent) => {
  // @ts-ignore
  const limit = Number(e.target!.value!);

  emit('limit-changed', limit);
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
</template>

<style scoped>
.configuration {
  border-radius: 4px;
  border: 1px solid grey;
  padding: 4px;
  margin-bottom: 5px;
}
</style>