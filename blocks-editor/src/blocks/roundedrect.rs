pub fn drawRoundedRectangle (startX:i8, basewidth:i8, width0:i8, radius:i8, baseheight:i8, startY:i8)->String {
      let offset = startY;
      let width = startX + basewidth;
      let start = startX;
      let sheight = baseheight / 2;//
      let height = sheight + offset; //25
      let startY0 = sheight + offset; //25;
      let par1 = 0 + offset;
      let par2 = 0 + offset;
      let par3 = sheight * 2 + offset; //50
      let par4 = sheight * 2 + offset;
      let result:String=format!("M{:?} {:?}
  C{:?} {:?} {:?} {:?} {:?} {:?}
  H{:?}
  C{:?} {:?} {:?} {:?} {:?} {:?}
  V{:?}
  C{:?} {:?} {:?} {:?} {:?} {:?}
  H{:?}
  C{:?} {:?} {:?} {:?} {:?} {:?}
  V{:?}
  Z",start,startY0,
    start,par1,start + radius, par2,  start + 2 * radius, par2,
    width,
            width + radius,par2, width + 2 * radius, par1,width + 2 * radius,height,
            height,
             width + 2 * radius, par3,width + radius,par4 ,width, par4,
            start + 2 * radius,
            start + radius,par4, start , par3 , start, height,
            height
      );
    result
}
