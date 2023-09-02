export default function Multiple() {
    const [formData, setFormData] = useState({email: "", password: ""});
  
    const handleChange = (event: { target: { name: any; value: any; }; }) => {
      const { name, value } = event.target;
      setFormData((prevFormData: any) => ({ ...prevFormData, [name]: value }));
    }
  
    const handleSubmit = (event) => {
        event.preventDefault();
        passchecker(formData.email, formData.password)
    };
  
    return (
      <form onSubmit={handleSubmit}>
        <label htmlFor="email">Email:</label>
        <input type="email" id="email" name="email" value={formData.email} onChange={handleChange}/>
        <br></br>
  
        <label htmlFor="password">Password:</label>
        <input type="password" id="password" name="password" value={formData.password} onChange={handleChange}/>
        <br></br>
        
        <button type="submit">Submit</button>
      </form>
    );
  }
  