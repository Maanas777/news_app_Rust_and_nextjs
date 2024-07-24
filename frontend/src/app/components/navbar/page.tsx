'use client'

import React, { useState } from "react";
import { FaBars, FaTimes } from "react-icons/fa";



const page = () => {


const [toggle, settoggle] = useState(false)

const togglehandle=()=>{
    settoggle(!toggle)
}
    
  return (
    
    <div className=" bg-sky-400">
       
      <div className="container py-5 px-6">
        <ul className="flex flex-row justify-between">
          <li className="mr-auto font-bold text-2xl ">News App</li>
          <div className="hidden md:flex space-x-8">
            <li className="font-bold"> Links</li>
            <li className="font-bold">Login</li>
          </div>
          <div className="md:hidden">
            <button onClick={togglehandle}>
             {toggle?<FaTimes/>:<FaBars/>}
            </button>
          </div>
        </ul>
      {toggle&&(
        <div className="md:hidden mt-4 space-y-2">
            <li className="font-bold">Links</li>
            <li className="font-bold">Login</li>

        </div>


      )}

      </div>
    </div>
  );
};

export default page;
