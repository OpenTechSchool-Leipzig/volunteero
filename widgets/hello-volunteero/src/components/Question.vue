<template>
  <li class="question">
    <span>{{ text }}</span>
    <button>ja</button>
    <button>nein</button>
  </li>
</template>

<script>
export default {
  props: {
    id: Number,
    text: String,
    categroy: Array
  }
};
</script>

<style></style>
