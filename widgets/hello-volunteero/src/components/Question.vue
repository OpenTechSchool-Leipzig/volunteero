<template>
  <li class="question">
    <span>{{ text }}</span>
    <button @click="emitChoice('yes')">ja</button>
    <button @click="emitChoice('no')">nein</button>
  </li>
</template>

<script>
export default {
  name: "Question",
  props: {
    id: Number,
    text: String,
    category: Array
  },
  methods: {
    emitChoice(choice) {
      this.$emit(choice, this.category);
    }
  }
};
</script>

<style></style>
