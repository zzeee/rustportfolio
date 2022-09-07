
let arr=[]
let tms=new Date().getTime()
for (let i=0;i<=3599;i++){
    let bias=Math.round(Math.random()*200)-100;
    let timestamp=tms+i+bias
    let ifc=Math.round(Math.random()*5)
    let line={provider_id:10, key:20020021, value: parseFloat(Math.round(Math.random()*10)+0.01),timestamp}
if (ifc!==3) arr.push(line)
  //  console.log('ll',line)
}
let fs=require('fs')
fs.writeFileSync('example.json',JSON.stringify(arr))
//console.log(arr)