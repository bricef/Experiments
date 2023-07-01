<script lang="ts">
import * as d3 from 'd3';

const parseTime = d3.timeParse("%d-%b-%y");

export default {

  data() {
    return {
      svg: null,
      width: 800,
      height: 500,
      items:[
          { date: "24-Apr-07", amount: 93.24 },
          { date: "25-Apr-07", amount: 95.35 },
          { date: "26-Apr-07", amount: 98.84 },
          { date: "27-Apr-07", amount: 99.92 },
          { date: "30-Apr-07", amount: 99.8 },
          { date: "1-May-07", amount: 99.47 },
          { date: "2-May-07", amount: 100.39 },
          { date: "3-May-07", amount: 100.4 },
          { date: "4-May-07", amount: 100.81 },
          { date: "7-May-07", amount: 103.92 },
          { date: "8-May-07", amount: 105.06 },
          { date: "9-May-07", amount: 106.88 },
          { date: "10-May-07", amount: 107.34 },
        ]
    }
  },

  methods: {
    addItem(){
      console.log("up")
      this.items.push({ date: "11-May-07", amount: 107.34 })
      this.$forceUpdate()
    },
  },

  mounted(){
    this.svg = d3.create("svg").attr("width", this.width).attr("height", this.height).append("g")
  },

  render(){
    let data = this.$data.items;
    
    const x = d3
          .scaleTime()
          .domain(
            d3.extent(data, function (d) {
              return parseTime(d.date);
            })
          )
          .rangeRound([0, this.width]);

    const y = d3
          .scaleLinear()
          .domain(
            d3.extent(data, function (d) {
              return d.amount;
            })
          )
          .rangeRound([this.height, 0]);

    const line = d3.line()
      .x(function (d) {
        return x(parseTime(d.date));
      })
      .y(function (d) {
        return y(d.amount);
      });

    this.svg.append("g")
        .attr("transform", "translate(0," + this.height + ")")
        .call(d3.axisBottom(x));

    this.svg.append("g")
        .call(d3.axisLeft(y))
        .append("text")
        .attr("fill", "#000")
        .attr("transform", "rotate(-90)")
        .attr("y", 6)
        .attr("dy", "0.71em")
        .attr("text-anchor", "end")
        .text("Price ($)");

    this.svg.append("path")
          .datum(data)
          .attr("fill", "none")
          .attr("stroke", "steelblue")
          .attr("stroke-width", 1.5)
          .attr("d", line);
    
      return this.svg
    }
}

</script>

<template>
    <div>
        <h2>Vue.js and D3 Line Chart</h2>
        <button @click="addItem()">Add item</button>
        <svg></svg>
    </div>
</template>
