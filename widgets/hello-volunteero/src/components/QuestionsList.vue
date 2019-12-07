<template>
  <div class="questions">
    <h2>Welche Fähigkeiten kannst du einbringen?</h2>
    <ul>
      <transition mode="out-in">
        <Question
          :key="activeQuestion.id"
          v-bind="activeQuestion"
          @yes="choseCategory"
          @no="addQuestion"
        />
      </transition>
    </ul>

    <button @click="resetQuestions">
      Neu Fragen gefällig?
    </button>
  </div>
</template>

<script>
import talents from "../Talentkarte.json";
import Question from "@/components/Question.vue";
export default {
  components: {
    Question
  },
  data() {
    return {
      selectedTalents: [],
      chosenCategories: []
    };
  },
  computed: {
    activeQuestion() {
      return talents.find(
        t => t.id === this.selectedTalents[this.selectedTalents.length - 1]
      );
    }
  },
  methods: {
    getRandId() {
      //make sure to get one talent for each category
      return 1 + Math.round(Math.random() * (talents.length - 1));
    },
    choseCategory(cats) {
      this.chosenCategories = [...this.chosenCategories, ...cats];
      this.addQuestion();
    },
    addQuestion() {
      if (this.selectedTalents.length < 3) {
        let newId = this.getRandId();
        while (this.selectedTalents.includes(newId)) {
          newId = this.getRandId();
        }
        this.selectedTalents = [...this.selectedTalents, newId];
      } else {
        // remove duplicates form chosenCategories through creating a new Set from the Array and parse it back using the spread operator.
        this.$emit("setResults", [...new Set(this.chosenCategories)]);
      }
    },
    resetQuestions() {
      this.selectedTalents = [this.getRandId()];
      this.chosenCategories = [];
      this.$emit("resetResults");
    }
  },
  mounted() {
    const randId = this.getRandId();
    this.selectedTalents = [...this.selectedTalents, randId];
  }
};
</script>

<style lang="scss">
.questions {
  display: flex;
  flex-direction: column;
  align-items: center;
  ul {
    list-style: none;
    padding: 0;
    width: 100%;
    margin-bottom: 60px;
  }
  h2 {
    font-size: 1.3rem;
    color: #009ee0;
    color: var(--primary-color);
  }
  button {
    font-size: 1rem;
    width: 48%;
    padding: 0.5rem;
    border: 1px solid #009ee0;
    border-color: var(--primary-color);
    border-radius: 0.3rem;
    background-color: transparent;
    transition: all 0.3s ease;

    &:hover,
    &:focus {
      background-color: #009ee0;
      background-color: var(--primary-color);
    }
  }
}
</style>
