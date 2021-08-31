const API_ADDRESS = "/api/todos/";

function timeDifference(current, previous) {
    
    var msPerMinute = 60 * 1000;
    var msPerHour = msPerMinute * 60;
    var msPerDay = msPerHour * 24;
    var msPerMonth = msPerDay * 30;
    var msPerYear = msPerDay * 365;
    
    var elapsed = current - previous;
    
    if (elapsed < msPerMinute) {
         return Math.round(elapsed/1000) + ' seconds ago';   
    }
    
    else if (elapsed < msPerHour) {
         return Math.round(elapsed/msPerMinute) + ' minutes ago';   
    }
    
    else if (elapsed < msPerDay ) {
         return Math.round(elapsed/msPerHour ) + ' hours ago';   
    }

    else if (elapsed < msPerMonth) {
         return 'approximately ' + Math.round(elapsed/msPerDay) + ' days ago';   
    }
    
    else if (elapsed < msPerYear) {
         return 'approximately ' + Math.round(elapsed/msPerMonth) + ' months ago';   
    }
    
    else {
         return 'approximately ' + Math.round(elapsed/msPerYear ) + ' years ago';   
    }
}

class TodosModel {
    constructor(event_pump) {
        this.$event_pump = $('body');
    }

    readTodos() {
        let ajax_options = {
            type: 'GET',
            url: API_ADDRESS,
            accepts: 'application/json',
            dataType: 'json'
        };
        $.ajax(ajax_options)
            .done((data) => {
                this.$event_pump.trigger('model_read_success', [data]);
            }
            )
            .fail((xhr, textStatus, errorThrown) => {
                console.log(errorThrown);
            })
    }

    createPerson(task) {
        let ajax_options = {
            type: 'POST',
            url: API_ADDRESS,
            contentType: "application/json; charset=utf-8",
            dataType: 'text',
            data: JSON.stringify({ name: task })
        };
        $.ajax(ajax_options)
            .done((reply) => {
                this.$event_pump.trigger('model_state_changed', []);
            })
            .fail((xhr, textStatus, errorThrown) => {
                console.log(errorThrown);
            })
    }

    updatePerson(id, task) {
        let ajax_options = {
            type: 'PUT',
            url: API_ADDRESS + id,
            contentType: "application/json; charset=utf-8",
            data: JSON.stringify({ name: task })

        };
        $.ajax(ajax_options)
            .done((reply) => {
                this.$event_pump.trigger('model_state_changed', []);
            })
            .fail((xhr, textStatus, errorThrown) => {
                console.log(errorThrown);
            })
    }

    deletePerson(id) {
        let ajax_options = {
            type: 'DELETE',
            url: API_ADDRESS + id,
        };
        $.ajax(ajax_options)
            .done((reply) => {
                this.$event_pump.trigger('model_state_changed', []);
            })
            .fail((xhr, textStatus, errorThrown) => {
                console.log(errorThrown);
            })
    }

    clearAllPeople() {
        let ajax_options = {
            type: 'DELETE',
            url: API_ADDRESS,
        };
        $.ajax(ajax_options)
            .done((reply) => {
                this.$event_pump.trigger('model_state_changed', []);
            })
            .fail((xhr, textStatus, errorThrown) => {
                console.log(errorThrown);
            })
    }

}

class TodoView {
    constructor() { }

    build_table(todos) {
        let rows = '';

        // clear the table
        $('.people table > tbody').empty();

        //confirm there is a people array
        if (todos) {
            for (let i = 0, l = todos.length; i < l; i++) {
                rows += `<tr>
                <td class="select"><input type="radio" id="person${i}" value=${todos[i].id} name="peopleRadios"></td>
                <td class="task-num">${i+1}</td>
                <td class="fname">${todos[i].name}</td>
                <td>${timeDifference(Date.now(), Date.parse(todos[i].timestamp))}</td></tr>`;
            }
            $('.people table > tbody').append(rows);
            // update it so the first element is checked
            $('input[type="radio"][name="peopleRadios"]').first().prop('checked', true);
        }
    }
}

class PeopleController {
    constructor(model, view) {
        this.$event_pump = $('body');
        this.model = model;
        this.view = view;

        //create event handlers
        this.create_events()

        //initialize handlers
        this.initialize_model_events()

        //get people from the model
        this.model.readTodos();
    }

    // Validate input
    validate(fname, lname) {
        return fname !== "" && lname !== "";
    }

    // Get selected Radio Button
    get_selected_id() {
        return $('input[name=peopleRadios]:checked').val();
    }

    create_events() {

        let $task = $('#task');

        //creating a new person
        $('#create').click((e) => {
            let task = $task.val();
            this.model.createPerson(task);
        });

        // deleting a person
        $('#delete').click((e) => {
            e.preventDefault();
            this.model.deletePerson(this.get_selected_id())
        });

        // updating a person
        $('#update').click((e) => {
            let task = $task.val();
            this.model.updatePerson(this.get_selected_id(), task)
        })

        // Clearing the Text Fields 
        $('#reset').click((e) => {
            this.model.clearAllPeople()
        })
    }

    initialize_model_events() {
        // Handle the model events
        this.$event_pump.on('model_read_success', (e, data) => {
            this.view.build_table(data);
        });

        // Handle the model events
        this.$event_pump.on('model_state_changed', (e) => {
            this.model.readTodos(); // on a successful creation readback the table so it's updated
        });
    }
}

const app = new PeopleController(new TodosModel(), new TodoView());