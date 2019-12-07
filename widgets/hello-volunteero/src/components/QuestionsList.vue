<template>
  <ul>
    <QuestionItem
      v-for="talent in talentObjects"
      :key="talent.id"
      v-bind="talent"
    />
  </ul>
</template>

<script>
import talents from "../Talentkarte.json";
import QuestionItem from "@/components/Question.vue";
export default {
  compoments: {
    QuestionItem
  },
  data() {
    return {
      selectedTalents: []
    };
  },
  computed: {
    talentObjects() {
      return this.selectedTalents.map(id => talents.find(t => t.id === id));
    }
  },
  methods: {
    getRandId() {
      return Math.round(Math.random() * 21);
    }
  },
  mounted() {
    const randId = this.getRandId();
    this.selectedTalents = [...this.selectedTalents, randId];
  }
};
</script>

<style></style>
