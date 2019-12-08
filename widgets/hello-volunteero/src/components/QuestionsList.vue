<template>
  <div class="questions">
    <template v-if="!showResults">
      <h2>Welche Fähigkeiten kannst du einbringen?</h2>
      <ul>
        <transition>
          <Question
            :key="activeQuestion.id"
            v-bind="activeQuestion"
            @yes="choseCategory"
            @no="addQuestion"
          />
        </transition>
      </ul>
    </template>
    <button @click="resetQuestions">Neu starten</button>
  </div>
</template>

<script>
import talents from "../Talentkarte.json";
import Question from "@/components/Question.vue";
export default {
  components: {
    Question
  },
  props: {
    showResults: Boolean
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
        // emove duplicates form chosenCategories through creating a new Set from the Array and parse it back using the spread operator.
        this.$emit("setResults", [...new Set(this.chosenCategories)]);
      }
    },
    resetQuestions() {
      this.selectedTalents = [this.getRandId()];
      this.chosenCategories = [];
      this.$emit("resetResults");
    }
  },
  created() {
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
    position: relative;
    list-style: none;
    padding: 0;
    width: 100%;
    height: 120px;
    margin-bottom: 30px;
  }
  button {
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
  }
}
</style>
