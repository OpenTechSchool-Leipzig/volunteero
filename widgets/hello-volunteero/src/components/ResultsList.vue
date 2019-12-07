<template>
  <div>
    <Result v-for="result in results" :key="result.organisation.id" v-bind="result" />
  </div>
</template>

<script>
import Result from "@/components/Result";

export default {
  components: {
    Result
  },
  props: {
    category: {
      default: () => []
    }
  },
  async mounted() {
    if (this.category) {
      return;
    }

    const categories = this.category.join(",category:");
    try {
      const res = await fetch(
        `https://volunteero.noidea.info/opportunities?labels=category:${categories}`
      );
      if (res.status === 200) {
        const json = await res.json();
        this.results = json;
      }
    } catch (error) {
      console.log(error);
    }
  },
  data() {
    return {
      results: []
    };
  }
};
</script>

<style>
</style>

