<template>
  <transition name="modal" appear>
    <div class="modal">
      <div class="modal-background" @click.prevent="emitClose"></div>
      <div class="modal-content">
        <button @click.prevent="emitClose" class="modal-close" aria-label="close">
          <Cross />
        </button>
        <QuestionList
          @setResults="onSetResults"
          @resetResults="onResetResults"
          :showResults="showResults"
        />
        <p v-if="showResults && chosenCategories.length === 0">Ooops probier's noch mal ;)</p>
        <ResultsList v-if="showResults && chosenCategories.length" :categories="chosenCategories" />
      </div>
    </div>
  </transition>
</template>

<script>
import QuestionList from "./QuestionsList";
import ResultsList from "./ResultsList";
import Cross from "@/svg/Cross";

export default {
  components: {
    QuestionList,
    ResultsList,
    Cross
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
    },
    onResetResults() {
      this.showResults = false;
      this.chosenCategories = [];
    }
  },
  befoemount() {
    window.addEventListener("keyup", this.onEscapeKeyUp);
  },
  beforeDestroy() {
    window.emoveEventListener("keyup", this.onEscapeKeyUp);
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
  z-index: 99;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}

.modal-close {
  position: absolute;
  right: 5px;
  top: 5px;
  border-radius: 50%;
  border: 1px solid #009ee0;
  border-color: var(--primary-color);
  background-color: transparent;
  width: 35px;
  height: 35px;
  transition: all 0.3s ease;
  cursor: pointer;

  &:hover,
  &:focus {
    background-color: #009ee0;
    background-color: var(--primary-color);
    border-color: white;
    svg {
      fill: white;
    }
  }

  &:disabled {
    background-color: lightgray;
    border-color: darkgrey;
    svg {
      fill: darkgrey;
    }
  }

  svg {
    fill: #009ee0;
    fill: var(--primary-color);
    display: block;
    width: 100%;
    height: 100%;
  }
}

.modal-content {
  border-radius: 0.5em;
  min-height: 300px;
  width: 100%;
  max-width: 500px;
  padding: 1em;
  top: 0;
  position: fixed;
  background-color: white;
  left: 50%;
  transform: translateX(-50%);
  overflow: hidden;
  z-index: 99;

  @media (min-width: 800px) {
    top: 10%;
  }
}

.modal-enter,
.modal-leave-to {
  opacity: 0.9;
  .modal-background {
    opacity: 0;
  }
  .modal-content,
  .modal-close {
    transform: translate(-50%, -70vh);
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
