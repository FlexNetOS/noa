'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { Switch } from '@/components/ui/switch';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { Checkbox } from '@/components/ui/checkbox';
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group';
import { AlertCircle, CheckCircle2, FileText } from 'lucide-react';
import { motion } from 'framer-motion';

/**
 * FormBuilder Widget - Dynamic form generator
 * 
 * Features:
 * - Multiple field types (text, number, select, checkbox, etc.)
 * - Validation with error messages
 * - Conditional field visibility
 * - Form state management
 * - Submit handling with async support
 * - Reset functionality
 * 
 * Rust/Dioxus Translation:
 * - Use Dioxus form components
 * - Implement validation with validator crate
 * - Use controlled inputs with signals
 * - Map to async form submission
 */

type FieldType = 'text' | 'number' | 'email' | 'password' | 'textarea' |
  'select' | 'checkbox' | 'radio' | 'switch' | 'date';

interface FieldOption
{
  label: string;
  value: string;
}

interface FieldDefinition
{
  name: string;
  label: string;
  type: FieldType;
  placeholder?: string;
  required?: boolean;
  options?: FieldOption[]; // For select/radio
  validation?: ( value: any ) => string | null;
  visible?: ( formData: Record<string, any> ) => boolean;
  defaultValue?: any;
}

interface FormBuilderconfigs
{
  title?: string;
  fields: FieldDefinition[];
  submitLabel?: string;
  onSubmit: ( data: Record<string, any> ) => Promise<void> | void;
  onReset?: () => void;
}

interface FormBuilderProps
{
  configs: FormBuilderconfigs;
  className?: string;
}

export function FormBuilder ( { configs, className = '' }: FormBuilderProps )
{
  const { title = 'Form', fields, submitLabel = 'Submit', onSubmit, onReset } = configs;

  // Initialize form data with defaults
  const initialData = fields.reduce( ( acc, field ) =>
  {
    acc[ field.name ] = field.defaultValue ?? '';
    return acc;
  }, {} as Record<string, any> );

  const [ formData, setFormData ] = useState<Record<string, any>>( initialData );
  const [ errors, setErrors ] = useState<Record<string, string>>( {} );
  const [ isSubmitting, setIsSubmitting ] = useState( false );
  const [ submitStatus, setSubmitStatus ] = useState<'idle' | 'success' | 'error'>( 'idle' );

  // Update field value
  const updateField = ( name: string, value: any ) =>
  {
    setFormData( prev => ( { ...prev, [ name ]: value } ) );
    // Clear error for this field
    if ( errors[ name ] )
    {
      setErrors( prev =>
      {
        const next = { ...prev };
        delete next[ name ];
        return next;
      } );
    }
  };

  // Validate form
  const validateForm = (): boolean =>
  {
    const newErrors: Record<string, string> = {};

    for ( const field of fields )
    {
      // Skip if not visible
      if ( field.visible && !field.visible( formData ) ) continue;

      const value = formData[ field.name ];

      // Required validation
      if ( field.required && ( !value || value === '' ) )
      {
        newErrors[ field.name ] = `${ field.label } is required`;
        continue;
      }

      // Custom validation
      if ( field.validation && value )
      {
        const error = field.validation( value );
        if ( error )
        {
          newErrors[ field.name ] = error;
        }
      }
    }

    setErrors( newErrors );
    return Object.keys( newErrors ).length === 0;
  };

  // Handle submit
  const handleSubmit = async ( e: React.FormEvent ) =>
  {
    e.preventDefault();

    if ( !validateForm() )
    {
      setSubmitStatus( 'error' );
      return;
    }

    setIsSubmitting( true );
    setSubmitStatus( 'idle' );

    try
    {
      await onSubmit( formData );
      setSubmitStatus( 'success' );
      setTimeout( () => setSubmitStatus( 'idle' ), 3000 );
    } catch ( error )
    {
      setSubmitStatus( 'error' );
      setErrors( { _form: 'Submission failed. Please try again.' } );
    } finally
    {
      setIsSubmitting( false );
    }
  };

  // Handle reset
  const handleReset = () =>
  {
    setFormData( initialData );
    setErrors( {} );
    setSubmitStatus( 'idle' );
    if ( onReset ) onReset();
  };

  // Render field based on type
  const renderField = ( field: FieldDefinition ) =>
  {
    const { name, label, type, placeholder, required, options } = field;
    const value = formData[ name ] ?? '';
    const error = errors[ name ];

    // Check visibility
    if ( field.visible && !field.visible( formData ) )
    {
      return null;
    }

    const fieldId = `field-${ name }`;

    return (
      <motion.div
        key={ name }
        initial={ { opacity: 0, y: 10 } }
        animate={ { opacity: 1, y: 0 } }
        className="space-y-2"
      >
        <Label htmlFor={ fieldId } className="flex items-center gap-1">
          { label }
          { required && <span className="text-destructive">*</span> }
        </Label>

        {/* Text-based inputs */ }
        { [ 'text', 'email', 'password', 'number', 'date' ].includes( type ) && (
          <Input
            id={ fieldId }
            type={ type }
            value={ value }
            onChange={ ( e ) => updateField( name, e.target.value ) }
            placeholder={ placeholder }
            className={ error ? 'border-destructive' : '' }
          />
        ) }

        {/* Textarea */ }
        { type === 'textarea' && (
          <Textarea
            id={ fieldId }
            value={ value }
            onChange={ ( e ) => updateField( name, e.target.value ) }
            placeholder={ placeholder }
            className={ error ? 'border-destructive' : '' }
            rows={ 4 }
          />
        ) }

        {/* Select */ }
        { type === 'select' && options && (
          <Select value={ value } onValueChange={ ( val ) => updateField( name, val ) }>
            <SelectTrigger id={ fieldId } className={ error ? 'border-destructive' : '' }>
              <SelectValue placeholder={ placeholder || 'Select an option' } />
            </SelectTrigger>
            <SelectContent>
              { options.map( ( opt ) => (
                <SelectItem key={ opt.value } value={ opt.value }>
                  { opt.label }
                </SelectItem>
              ) ) }
            </SelectContent>
          </Select>
        ) }

        {/* Radio Group */ }
        { type === 'radio' && options && (
          <RadioGroup value={ value } onValueChange={ ( val ) => updateField( name, val ) }>
            { options.map( ( opt ) => (
              <div key={ opt.value } className="flex items-center space-x-2">
                <RadioGroupItem value={ opt.value } id={ `${ fieldId }-${ opt.value }` } />
                <Label htmlFor={ `${ fieldId }-${ opt.value }` } className="font-normal">
                  { opt.label }
                </Label>
              </div>
            ) ) }
          </RadioGroup>
        ) }

        {/* Checkbox */ }
        { type === 'checkbox' && (
          <div className="flex items-center space-x-2">
            <Checkbox
              id={ fieldId }
              checked={ value }
              onCheckedChange={ ( checked ) => updateField( name, checked ) }
            />
            <Label htmlFor={ fieldId } className="font-normal">
              { placeholder || label }
            </Label>
          </div>
        ) }

        {/* Switch */ }
        { type === 'switch' && (
          <div className="flex items-center space-x-2">
            <Switch
              id={ fieldId }
              checked={ value }
              onCheckedChange={ ( checked ) => updateField( name, checked ) }
            />
            <Label htmlFor={ fieldId } className="font-normal">
              { placeholder || label }
            </Label>
          </div>
        ) }

        {/* Error message */ }
        { error && (
          <motion.p
            initial={ { opacity: 0 } }
            animate={ { opacity: 1 } }
            className="text-sm text-destructive flex items-center gap-1"
          >
            <AlertCircle className="h-3 w-3" />
            { error }
          </motion.p>
        ) }
      </motion.div>
    );
  };

  return (
    <Card className={ className }>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <FileText className="h-5 w-5" />
          { title }
        </CardTitle>
      </CardHeader>
      <CardContent>
        <form onSubmit={ handleSubmit } className="space-y-6">
          {/* Fields */ }
          { fields.map( renderField ) }

          {/* Form-level error */ }
          { errors._form && (
            <div className="p-3 bg-destructive/10 border border-destructive/20 rounded-lg">
              <p className="text-sm text-destructive flex items-center gap-2">
                <AlertCircle className="h-4 w-4" />
                { errors._form }
              </p>
            </div>
          ) }

          {/* Success message */ }
          { submitStatus === 'success' && (
            <motion.div
              initial={ { opacity: 0, y: -10 } }
              animate={ { opacity: 1, y: 0 } }
              className="p-3 bg-green-500/10 border border-green-500/20 rounded-lg"
            >
              <p className="text-sm text-green-600 dark:text-green-400 flex items-center gap-2">
                <CheckCircle2 className="h-4 w-4" />
                Form submitted successfully!
              </p>
            </motion.div>
          ) }

          {/* Actions */ }
          <div className="flex gap-2 pt-4">
            <Button
              type="submit"
              disabled={ isSubmitting }
              className="flex-1"
            >
              { isSubmitting ? 'Submitting...' : submitLabel }
            </Button>
            <Button
              type="button"
              variant="outline"
              onClick={ handleReset }
              disabled={ isSubmitting }
            >
              Reset
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  );
}
