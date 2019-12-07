<template>
  <transition name="modal" appear>
    <div class="modal">
      <div class="modal-background" @click.prevent="emitClose"></div>
      <div class="modal-content">
        <QuestionList @setResults="onSetResults" />
        <ul v-if="showResults">
          <li v-for="(cat, key) in chosenCategories" :key="key">{{ cat }}</li>
        </ul>
        <ResultsList v-if="showResults" />
      </div>
      <button
        @click.prevent="emitClose"
        class="modal-close"
        aria-label="close"
      ></button>
    </div>
  </transition>
</template>

<script>
import QuestionList from "./QuestionsList";
import ResultsList from "./ResultsList";
export default {
  components: {
    QuestionList,
    ResultsList
  },
  data() {
    return {
      showResults: false,
      chosenCategories: []
    };
  },
  methods: {
    emitClose() {
      this.$emit("closeModal");
    },
    onEscapeKeyUp(event) {
      if (event.which === 27) {
        this.emitClose();
      }
    },
    onSetResults(cats) {
      this.showResults = true;
      this.chosenCategories = cats;
    }
  },
  beforeMount() {
    window.addEventListener("keyup", this.onEscapeKeyUp);
  },
  beforeDestroy() {
    window.removeEventListener("keyup", this.onEscapeKeyUp);
  }
};
</script>

<style lang="scss" scoped>
.modal {
  display: flex;
}

.modal-background {
  background-color: rgba(black, 0.7);
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}

.modal-content {
  border-radius: 0.5rem;
  min-height: 300px;
  width: 100%;
  max-width: 500px;
  padding: 0.75rem;
  top: 20%;
  position: fixed;
  background-color: white;
  left: 50%;
  transform: translateX(-50%);
}

.modal-enter,
.modal-leave-to {
  opacity: 0.9;
  .modal-background {
    opacity: 0;
  }
  .modal-content,
  .modal-close {
    transform: translateY(-70vh);
  }
}

.modal-enter-active,
.modal-leave-active {
  transition: all 0.8s ease;
  .modal-background,
  .modal-close,
  .modal-content {
    transition: all 0.8s ease;
  }
}
</style>
