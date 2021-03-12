        class State {
          constructor(){
            this.camera = false;
            this.loop = false;
           }
          get_camera(){
            this.set_camera();
            return this.camera;
          }
          run(){
            fetch('/' +this.get_camera())
               .catch(console.log(error));
          }
          set_camera(){
            this.camera = !this.camera;
          }
          set_loop(){
            this.loop = !this.loop;
         }
         sleep(ms) {
           return new Promise(resolve => setTimeout(resolve, ms));
         }
         loop(){
         this.set_loop();
         while (this.loop) {
             this.run();
             await this.sleep(Math.random() + 5000);
             }
        }
        }
        let state = new State();

