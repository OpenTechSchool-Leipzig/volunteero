<template>
  <div>
    <button @click="currentSlide--" :disabled="currentSlide === 0">Prev</button>
    <button @click="currentSlide++" :disabled="currentSlide === results.length">
      Next
    </button>
    <div class="result-container">
      <transition>
        <Result v-if="activeResult" :key="currentSlide" v-bind="activeResult" />
      </transition>
    </div>
  </div>
</template>

<script>
import Result from "@/components/Result";

export default {
  components: {
    Result
  },
  props: {
    categories: {
      default: () => []
    }
  },
  data() {
    return {
      results: [],
      currentSlide: 0
    };
  },
  computed: {
    activeResult() {
      if (!this.results) return null;
      return this.results[this.currentSlide];
    }
  },
  async mounted() {
    if (!this.categories) {
      return;
    }

    const joinedCategories = this.categories.join(",category:");
    try {
      const res = await fetch(
        `https://volunteero.noidea.info/opportunities?labels=category:${joinedCategories}`
      );
      if (res.status === 200) {
        const json = await res.json();
        this.results = json;
      }
    } catch (error) {
      console.log(error);
    }
  }
};
</script>
<style lang="scss">
.result-container {
  position: relative;
  width: 100%;
  height: 200px;
}
</style>
