<template>
  <li class="question">
    <div class="question-text">{{ text }}</div>
    <button @click="emitChoice('yes')" class="choice-btn">ja</button>
    <button class="choice-btn" @click="emitChoice('no')">nein</button>
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
      this.choice = choice;
      this.$emit(choice, this.category);
    }
  }
};
</script>

<style lang="scss">
.choice-btn {
  font-size: 1em;
  width: 48%;
  padding: 0.5em;
  border: 1px solid #009ee0;
  border-color: var(--primary-color);
  border-radius: 0.3em;
  background-color: transparent;
  transition: all 0.3s ease;

  &:hover,
  &:focus {
    background-color: #009ee0;
    background-color: var(--primary-color);
  }

  &:last-child {
    margin-left: 4%;
  }
}
.question {
  text-align: center;
  position: absolute;
  width: 100%;
  top: 0;
  left: 0;
  transition: all 0.3s;
  &.v-enter {
    transform: translateX(200%);
    opacity: 15%;
  }

  &.v-leave-to {
    transform: translateX(-200%);
    opacity: 15%;
  }
  &.v-enter-to,
  &.v-leave {
    transform: translateX(0);
    opacity: 100%;
  }
  &.v-enter-active,
  &.v-leave-active,
  &.v-move {
    transition: all 0.5s ease-out;
  }

  .question-text {
    font-size: 1.2em;
    padding: 1em;
  }
}
</style>
