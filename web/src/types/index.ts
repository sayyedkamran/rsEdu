// Auth
export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  username: string;
  role: string;
  role_title: string;
  organization_id: number | null;
  branch_id: number | null;
}

// User
export interface User {
  id: number;
  username: string;
  email: string;
  organization_id: number | null;
  branch_id: number | null;
  is_active: boolean;
}

// Organization
export interface Organization {
  id: number;
  name: string;
  name_urdu: string | null;
  logo_url: string | null;
  website: string | null;
  email: string | null;
  phone: string | null;
  city_id: number | null;
  address: string | null;
  is_active: boolean;
}

// Branch
export interface Branch {
  id: number;
  organization_id: number;
  name: string;
  name_urdu: string | null;
  code: string;
  city_id: number | null;
  area: string | null;
  address_line: string | null;
  postal_code: string | null;
  is_active: boolean;
}

// Student
export interface Student {
  id: number;
  user_id: number | null;
  organization_id: number;
  branch_id: number;
  first_name: string;
  last_name: string;
  date_of_birth: string;
  gender: string;
  roll_number: string | null;
  admission_date: string;
  is_active: boolean;
}

// Staff
export interface Staff {
  id: number;
  user_id: number | null;
  organization_id: number;
  branch_id: number | null;
  staff_type_id: number;
  first_name: string;
  last_name: string;
  date_of_birth: string | null;
  joining_date: string;
  is_active: boolean;
}

// Class
export interface Class {
  id: number;
  branch_id: number;
  master_class_id: number;
  master_section_id: number;
  academic_year_id: number;
  class_staff_id: number | null;
  capacity: number | null;
  is_active: boolean;
}

// Academic Year
export interface AcademicYear {
  id: number;
  organization_id: number;
  stream_id: number;
  title: string;
  start_date: string;
  end_date: string;
  description: string | null;
  is_active: boolean;
}

// Fee Bill
export interface FeeBill {
  id: number;
  student_id: number;
  branch_id: number;
  academic_year_id: number;
  fee_type_id: number;
  bill_month: number;
  bill_year: number;
  amount: number;
  discount_amount: number;
  late_fee: number;
  carry_forward: number;
  net_amount: number;
  amount_paid: number;
  balance: number;
  status: string;
  due_date: string;
  generated_at: string;
}

// Attendance
export interface Attendance {
  id: number;
  branch_id: number;
  class_id: number;
  student_id: number;
  marked_by: number;
  date: string;
  status: string;
  remarks: string | null;
}

// Province
export interface Province {
  id: number;
  name: string;
  name_urdu: string | null;
  code: string;
  is_active: boolean;
}

// City
export interface City {
  id: number;
  name: string;
  name_urdu: string | null;
  province_id: number;
  is_active: boolean;
}

// API Response wrapper
export interface ApiError {
  message: string;
  status: number;
}

// Pagination (for future use)
export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
}