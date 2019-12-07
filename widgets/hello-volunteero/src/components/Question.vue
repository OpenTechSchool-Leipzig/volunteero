<template>
  <li class="question">
    <span>{{ text }}</span>

    <button
      @click="emitChoice('yes')"
      class="choice-btn"
      :class="[choice === 'yes' ? 'selected-yes' : '']"
      :disabled="choice"
    >
      ja
    </button>
    <button
      class="choice-btn"
      @click="emitChoice('no')"
      :class="[choice === 'no' ? 'selected-no' : '']"
      :disabled="choice"
    >
      nein
    </button>
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
  data() {
    return {
      choice: null
    };
  },
  methods: {
    emitChoice(choice) {
      this.choice = choice;
      this.$emit(choice, this.category);
    }
  }
};
</script>

<style lang="scss">
.choice-btn {
  padding: 0.5rem;
  border-radius: 0.5rem;

  &:not(:last-child) {
    margin-right: 0.25rem;
  }
  &.selected-no {
    background-color: red;
  }
  &.selected-yes {
    background-color: green;
  }
}
</style>
