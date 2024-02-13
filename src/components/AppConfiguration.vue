<script setup lang="ts">
const emit = defineEmits([
    'limit-changed',
    'regular-wp-changed',
    'format-changed'
]);

const props = defineProps<{
  show_limit: number,
  regular_wp: 0 | 1 | 2,
  format: 0 | 1 | 2 | 3
}>();

const show_limit_changed = (e: InputEvent) => {
  // @ts-ignore
  const limit = Number(e.target!.value!);

  emit('limit-changed', limit);
}

const regular_wp_changed = (e: InputEvent) => {
  // @ts-ignore
  const regular_wp = Number(e.target!.value!);

  emit('regular-wp-changed', regular_wp);
}
const format_changed = (format: 0 | 1 | 2 | 3) => {
  emit('format-changed', format);
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
  .formats
    span フォーマット
    a(href="#" :class="props.format == 0 ? 'active' : ''" title="指定なし" @click.prevent="format_changed(0)")
      img(src="/any.svg")
    a(href="#" :class="props.format == 1 ? 'active' : ''" title="オールスター" @click.prevent="format_changed(1)")
      img(src="/all_star.svg")
    a(href="#" :class="props.format == 2 ? 'active' : ''" title="キーセレクション" @click.prevent="format_changed(2)")
      img(src="/key_selection.svg")
    a(href="#" :class="props.format == 3 ? 'active' : ''" title="ディーヴァセレクション" @click.prevent="format_changed(3)")
      img(src="/diva_selection.svg")
</template>

<style scoped lang="less">
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
  margin-left: 4px;
  font-size: 1.2rem;
  padding: 6px;
  outline: 0 solid transparent;
  border: 1px solid #686868;
  border-radius: 4px;
  box-shadow: none;
}

.formats {
  display: inline-block;

  a {
    margin-top: 4px;
    border: 1px solid transparent;
    background-color: transparent;
    padding: 5px 2px 0 2px;
    border-radius: 2px;

    &.active {
      border-color: red;
      background-color: pink;
    }

    img {
      width: 1rem;
      height: 1rem;
    }
  }
}
</style>