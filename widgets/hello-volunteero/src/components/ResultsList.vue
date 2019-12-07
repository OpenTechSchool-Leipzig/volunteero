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
  mounted() {
    fetch("https://volunteero.noidea.info/opportunities")
      .then(res => {
        console.log(res);
        if (res.response.data) {
          this.results = res.response.data;
        }
      })
      .catch(error => {
        console.log(error);
      });
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

