<template>
  <div class="bot">
    <div class="speech-bubble">Lust auf Ehrenamt?</div>
    <LamaSvg />
  </div>
</template>

<script>
import LamaSvg from "@/svg/Lama";

export default {
  components: { LamaSvg }
};
</script>

<style>
.bot {
  width: 150px;
  height: 200px;
}
.speech-bubble {
	position: relative;
  color: white;
  text-align: center;
  padding: 20px;
  margin-bottom: 5px; 
	background: rgb(0, 110, 201);
	border-radius: .4em;
  width: 100%
}

.speech-bubble:after {
	content: '';
	position: absolute;
	bottom: 0;
	left: 50%;
	width: 0;
	height: 0;
	border: 0.8em solid transparent;
	border-top-color:rgb(0, 110, 201);
	border-bottom: 0;
	border-left: 0;
	margin-left: -0.312em;
	margin-bottom: -0.8em;
}
</style>
