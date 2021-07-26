const API_ADDRESS = "/api/people/";

class PeopleModel {
    constructor(event_pump) {
        this.$event_pump = $('body');
    }

    readPeople() {
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

    createPerson(fname, lname) {
        let ajax_options = {
            type: 'POST',
            url: API_ADDRESS,
            contentType: "application/json; charset=utf-8",
            dataType: 'json',
            data: JSON.stringify({ fname: fname, lname: lname })
        };
        $.ajax(ajax_options)
            .done((reply) => {
                this.$event_pump.trigger('model_state_changed', []);
            })
            .fail((xhr, textStatus, errorThrown) => {
                console.log(errorThrown);
            })
    }

    updatePerson(id, fname, lname) {
        let ajax_options = {
            type: 'PUT',
            url: API_ADDRESS + id,
            contentType: "application/json; charset=utf-8",
            data: JSON.stringify({ fname: fname, lname: lname })
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


}

class PeopleView {
    constructor() { }

    build_table(people) {
        let rows = '';

        // clear the table
        $('.people table > tbody').empty();

        //confirm there is a people array
        if (people) {
            for (let i = 0, l = people.length; i < l; i++) {
                rows += `<tr>
                <td class="select"><input type="radio" id="person${i}" value=${people[i].id} name="peopleRadios"></td>
                <td class="fname">${people[i].fname}</td>
                <td class="lname">${people[i].lname}</td>
                <td>${people[i].timestamp}</td></tr>`;
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
        this.model.readPeople();
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

        let $fname = $('#fname'),
            $lname = $('#lname');

        //creating a new person
        $('#create').click((e) => {
            let fname = $fname.val(),
                lname = $lname.val();

            e.preventDefault();

            if (this.validate(fname, lname)) {
                this.model.createPerson(fname, lname)
            } else {
                alert('Problem with first or last name input');
            }
        });

        // deleting a person
        $('#delete').click((e) => {
            e.preventDefault();
            this.model.deletePerson(this.get_selected_id())
        });

        // updating a person
        $('#update').click((e) => {
            e.preventDefault();
            let fname = $fname.val(),
                lname = $lname.val();

            e.preventDefault();

            if (this.validate(fname, lname)) {
                this.model.updatePerson(this.get_selected_id(), fname, lname)
            } else {
                alert('Problem with first or last name input');
            }
        })

        // Clearing the Text Fields 
        $('#reset').click((e) => {
            e.preventDefault();
            $fname.val('');
            $lname.val('');
        })
    }

    initialize_model_events() {
        // Handle the model events
        this.$event_pump.on('model_read_success', (e, data) => {
            this.view.build_table(data);
        });

        // Handle the model events
        this.$event_pump.on('model_state_changed', (e) => {
            this.model.readPeople(); // on a successful creation readback the table so it's updated
        });
    }
}

const app = new PeopleController(new PeopleModel(), new PeopleView());