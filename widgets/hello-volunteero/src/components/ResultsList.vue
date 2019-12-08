<template>
  <div>
    <h2>Das könnte dich interessieren:</h2>
    <div class="result-container">
      <div class="slide-controls">
        <button class="slide-btn" @click="currentSlide--" :disabled="currentSlide === 0">
          <Arrow isLeft />
        </button>
        <button
          class="slide-btn"
          @click="currentSlide++"
          :disabled="currentSlide === results.length"
        >
          <Arrow />
        </button>
      </div>
      <transition>
        <Result v-if="activeResult" :key="currentSlide" v-bind="activeResult" />
      </transition>
    </div>
  </div>
</template>

<script>
import Result from "@/components/Result";
import Arrow from "@/svg/Arrow";

export default {
  components: {
    Result,
    Arrow
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
.slide-controls {
  position: absolute;
  width: 100%;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  justify-content: space-between;
  align-items: center;
  z-index: 1;
}
.slide-btn {
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
</style>
