<template>
  <div>
    <transition-group tag="ul">
      <Question
        v-for="talent in talentObjects"
        :key="talent.id"
        v-bind="talent"
        @yes="choseCategory"
        @no="addQuestion"
      />
    </transition-group>
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
    talentObjects() {
      return this.selectedTalents.map(id => talents.find(t => t.id === id));
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
    }
  },
  mounted() {
    const randId = this.getRandId();
    this.selectedTalents = [...this.selectedTalents, randId];
  }
};
</script>

<style></style>
