<template>
  <div class="bot" @click.prevent="openModal">
    <div class="message">Lust auf Ehrenamt?</div>
    <LamaSvg />
  </div>
</template>

<script>
import LamaSvg from "@/svg/Lama";

export default {
  components: { LamaSvg },
  methods: {
    openModal() {
      this.$emit("showModal");
    }
  }
};
</script>

<style>
.bot {
  width: 300px;
  height: 300px;
  cursor: pointer;
}
.message {
  color: black;
  border-radius: 10px;
  border: 1px solid black;
  width: 100%;
  padding: 20px;
}
</style>
