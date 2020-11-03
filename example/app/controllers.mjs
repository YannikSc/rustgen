class Controllers
{
    controllers = [];

    /**
     * @param {Function} controller
     */
    addController(controller)
    {
        this.controllers.push(controller);
    }

    /**
     *
     * @param {string} name
     */
    dispatch(name)
    {
        for (let controller of this.controllers) {
            if (controller.name === name) {
                controller();
            }
        }
    }
}

const controllers = new Controllers();

// Register Controllers
import test_3 from './controller/test_3.mjs';
controllers.addController(test_3);
import test_1 from './controller/test_1.mjs';
controllers.addController(test_1);
import test from './controller/test.mjs';
controllers.addController(test);


controllers.dispatch('test');