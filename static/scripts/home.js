const API_ADDRES = "/api/people/";

class PeopleModel {
    constructor() {}
}

class PeopleView {
    constructor() {}
}

class PeopleController {
    constructor(model, view) {
        this.model = model;
        this.view = view;
    }
}

const app = new PeopleController(new PeopleModel(), new PeopleView());