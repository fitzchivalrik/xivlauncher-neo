<template>
  <div id="main-view">
    <main-news/>
    <main-login/>
    <main-headlines/>
    <main-controls/>
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue"
import MainNews from "@/views/main/News.vue"
import MainHeadlines from "@/views/main/Headlines.vue"
import MainLogin from "@/views/main/Login.vue"
import MainControls from "@/views/main/Controls.vue"

export default defineComponent({
  name: 'main-view',
  props: {
    msg: String,
  },
  components: {
    MainControls,
    MainLogin,
    MainHeadlines,
    MainNews,
  },
})
</script>

<style lang="sass" scoped>
$width: 830px
$height: 340px

$windowPadding: 10px
$tableGap: 10px
$leftCol: 544px
$rightCol: $width - $leftCol - $tableGap - ($windowPadding * 2)
$topRow: 213px
$bottomRow: $height - $topRow - $tableGap - ($windowPadding * 2)

#main-view
  padding: $windowPadding
  display: grid
  grid-template-columns: $leftCol $rightCol
  grid-template-rows: $topRow $bottomRow
  column-gap: 10px
  row-gap: 10px

</style>