<template>
  <li class="question">
    <h2>Welche Fähigkeiten kannst du einbringen?</h2>
    <div class="question-text">{{ text }}</div>

    <button
      @click="emitChoice('yes')"
      class="choice-btn"
      :class="[choice === 'yes' ? 'selected-yes' : '']"
      :disabled="choice"
    >ja</button>
    <button
      class="choice-btn"
      @click="emitChoice('no')"
      :class="[choice === 'no' ? 'selected-no' : '']"
      :disabled="choice"
    >nein</button>
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
  font-size: 1rem;
  width: 48%;
  padding: 0.5rem;
  border: 1px solid black;
  border-radius: 0.3rem;
  background-color: white;

  &:last-child {
    margin-left: 4%;
  }
  &.selected-no {
    background-color: red;
  }
  &.selected-yes {
    background-color: green;
  }
}
.question {
  text-align: center;
  transition: all 0.3s;
  &.v-enter,
  &.v-leave-to {
    transform: translateY(200%);
    opacity: 15%;
  }
  &.v-enter-to,
  &.v-leave {
    transform: translateY(0);
    opacity: 100%;
  }
  &.v-enter-active,
  &.v-leave-active,
  &.v-move {
    transition: all 0.5s ease-out;
  }
  h2 {
    font-size: 1.3rem;
  }
  .question-text {
    font-size: 1.2rem;
    padding: 1rem;
  }
}
</style>
